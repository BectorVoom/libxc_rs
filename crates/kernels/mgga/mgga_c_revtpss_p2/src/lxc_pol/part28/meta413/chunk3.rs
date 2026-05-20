//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1563/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1563<F: Float>(t15339: F, t954: F, t4682: F, t964: F, t11404: F, t11409: F, t11507: F, t11548: F, t15263: F, t15267: F, t15274: F, t15277: F, t15280: F, t15284: F, t15287: F, t15290: F, t2943: F, t2968: F, t3007: F, t3012: F, t4652: F, t4674: F, t4685: F, t946: F, t974: F) -> F {
    let t15340 = t15339 * t954;
    let t15343 = t4682 * t964;
    let t15348 = F::cast_from(0.17315859105681463759e2_f64) * t3012 * t15263 + F::cast_from(0.10254018858216406658e4_f64) * t11507 * t15267 - F::new(4.0) * t11548 * t4652 + F::cast_from(0.64327917994770140268e2_f64) * t11404 * t4674 - F::new(4.0) * t2943 * t15274 - F::new(2.0) * t2943 * t15277 - F::cast_from(0.19298375398431042081e3_f64) * t11409 * t15280 + F::cast_from(0.64327917994770140268e2_f64) * t2968 * t15284 + F::new(6.0) * t2968 * t15287 + F::cast_from(0.35089341735807877242e1_f64) * t3012 * t15290 + F::new(1.0) * t946 * t15340 + F::cast_from(0.11696447245269292414e1_f64) * t15343 * t974 + F::cast_from(0.5848223622634646207e0_f64) * t4685 * t3007;
    t15348
}
