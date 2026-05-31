//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 614/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk614<F: Float>(t1563: F, t967: F, t2873: F, t506: F, t10: F, t127: F, t1511: F, t1519: F, t1540: F, t1542: F, t1555: F, t1558: F, t1561: F, t2862: F, t2865: F, t2868: F, t2876: F, t2879: F, t2881: F, t2886: F, t2891: F, t481: F, t496: F) -> (F, F, F) {
    let t2893 = t1563 * t967;
    let t2897 = t506 * t2873;
    let t2900 = -t1511 + t2862 + t1519 + t2865 + t2868 - t2876 + t1540 + t1542 / F::cast_from(6.0_f64) + t2879 / F::cast_from(6.0_f64) + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t496 * t10 * t2881 - t496 * t2886 / F::cast_from(2.0_f64) + t1555 + F::cast_from(0.73452e0_f64) * t1558 + t1561 + F::cast_from(0.73452e0_f64) * t2891 + F::cast_from(0.587616e1_f64) * t127 * t2893 * t481 - F::cast_from(0.146904e1_f64) * t127 * t2897;
    (t2893, t2897, t2900)
}
