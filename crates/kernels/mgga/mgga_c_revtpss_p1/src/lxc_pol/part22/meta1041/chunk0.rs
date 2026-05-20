//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3634/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3634<F: Float>(t12227: F, t12230: F, t3385: F, t6470: F, t12243: F, t20648: F, t16942: F, t3433: F, t5108: F, t16812: F, t5192: F, t1196: F, t3516: F, t6555: F) -> (F, F, F, F, F) {
    let t68779 = F::cast_from(0.51726012919273400301e3_f64) * t12227 * t6470 * t12230 * t3385;
    let t68781 = F::cast_from(0.64327917994770140268e2_f64) * t12243 * t20648;
    let t68784 = F::cast_from(0.32163958997385070134e2_f64) * t3433 * t5108 * t16942;
    let t68786 = F::cast_from(0.20508037716432813315e4_f64) * t5192 * t16812;
    let t68789 = F::cast_from(0.35089341735807877242e1_f64) * t1196 * t6555 * t3516;
    (t68779, t68781, t68784, t68786, t68789)
}
