//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2703/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2703<F: Float>(t105: F, t4283: F, t588: F, t100: F, t10217: F, t10236: F, t10243: F, t10247: F, t10250: F, t10251: F, t108: F, t13479: F, t13482: F, t1505: F, t1507: F, t22: F, t2344: F, t2357: F, t4269: F, t4270: F, t4274: F, t4279: F, t580: F, t656: F, t661: F, t97: F) -> F {
    let t49745 = F::cast_from(20.0_f64) * t105 * t4283 * t588;
    let t49760 = -F::cast_from(25.0_f64) / F::cast_from(9.0_f64) * t1507 * t10251 - F::cast_from(2200.0_f64) / F::cast_from(81.0_f64) * t10217 * t1505 - F::cast_from(25.0_f64) / F::cast_from(3.0_f64) * t656 * t13482 - F::cast_from(10.0_f64) * t97 * t100 * t22 + F::cast_from(50.0_f64) / F::cast_from(81.0_f64) * t1507 * t10243 + F::cast_from(10.0_f64) * t105 * t108 * t22 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t105 * t2357 * t580 * t661 - t49745 + F::cast_from(400.0_f64) / F::cast_from(27.0_f64) * t2344 * t4270 + F::cast_from(200.0_f64) / F::cast_from(9.0_f64) * t2344 * t4274 - F::cast_from(50.0_f64) / F::cast_from(9.0_f64) * t656 * t13479 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t97 * t4269 * t10236 - F::cast_from(50.0_f64) / F::cast_from(9.0_f64) * t1507 * t10247 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t105 * t4279 * t10250;
    t49760
}
