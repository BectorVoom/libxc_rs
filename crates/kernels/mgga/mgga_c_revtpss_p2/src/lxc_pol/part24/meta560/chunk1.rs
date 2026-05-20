//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1684/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1684<F: Float>(t88264: F, t964: F, t973: F, t981: F, t2986: F, t88351: F, t1642: F, t78704: F, t88445: F, t88448: F, t88451: F, t88481: F, t88580: F, t88584: F, t88586: F, t88588: F, t88590: F, t88592: F) -> (F, F, F, F) {
    let t88596 = F::cast_from(0.5848223622634646207e0_f64) * t981 * t964 * t88264 * t973;
    let t88600 = F::cast_from(0.35089341735807877242e1_f64) * t981 * t2986 * t88351 * t973;
    let t88602 = F::cast_from(0.23392894490538584828e1_f64) * t78704 * t1642;
    let t88603 = -t88580 + t88584 - t88445 + t88448 + t88451 - t88586 - t88588 + t88481 + t88590 - t88592 - t88596 + t88600 - t88602;
    (t88596, t88600, t88602, t88603)
}
