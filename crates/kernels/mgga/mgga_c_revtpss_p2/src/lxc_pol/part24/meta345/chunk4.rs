//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1202/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1202<F: Float>(t23694: F, t964: F, t973: F, t981: F, t1621: F, t6157: F, t954: F, t23451: F, t11509: F, t11507: F, t15104: F, t15413: F, t1622: F, t19173: F, t23461: F, t23463: F, t23465: F, t23469: F, t23549: F, t23552: F, t23564: F, t23567: F, t2968: F, t3012: F, t4647: F, t6158: F, t6174: F, t6190: F, t965: F) -> (F, F, F, F, F, F, F, F) {
    let t23696 = t964 * t23694 * t973;
    let t23698 = F::cast_from(0.5848223622634646207e0_f64) * t981 * t23696;
    let t23705 = t6157 * t1621;
    let t23706 = t23705 * t954;
    let t23711 = t23451 * t973;
    let t23714 = t23694 * t973;
    let t23717 = t23451 * t11509;
    let t23720 = -t23461 - t23463 - t23465 + t23469 - t23549 - t23552 + F::new(3.0) * t19173 * t1622 + F::new(3.0) * t4647 * t6174 + t23564 - t23567 - F::new(6.0) * t15104 * t6158 + F::new(6.0) * t2968 * t23706 - F::cast_from(0.35089341735807877242e1_f64) * t15413 * t6190 + F::cast_from(0.35089341735807877242e1_f64) * t3012 * t23711 + F::cast_from(0.5848223622634646207e0_f64) * t965 * t23714 + F::cast_from(0.10254018858216406658e4_f64) * t11507 * t23717;
    (t23696, t23698, t23705, t23706, t23711, t23714, t23717, t23720)
}
