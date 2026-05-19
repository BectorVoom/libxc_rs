//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1088/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1088<F: Float>(t1347: F, t2479: F, t1652: F, t8264: F, t2471: F, t794: F, t2211: F, t5249: F, t1356: F, t1550: F, t2604: F, t27124: F, t36448: F, t40683: F, t40685: F, t40688: F, t40690: F, t40695: F, t40700: F, t40703: F, t40706: F, t4965: F, t5928: F, t739: F, t8265: F, t9437: F, t9531: F) -> (F, F, F, F) {
    let t43680 = t1347 * t2479;
    let t43685 = t8264 * t1652;
    let t43692 = t2471 * t794;
    let t43698 = t2211 * t5249;
    let t43708 = t43680 - F::cast_from(0.5107751987195740728e-4_f64) * t40683 + F::cast_from(0.11974241701863808564e0_f64) * t739 * t2211 * t27124 + F::cast_from(0.79828278012425390428e-1_f64) * t1356 * t43685 - F::cast_from(0.11974241701863808564e0_f64) * t40685 - F::cast_from(0.5987120850931904282e-1_f64) * t40688 + F::cast_from(0.79828278012425390428e-1_f64) * t4965 * t9531 - F::cast_from(0.11974241701863808564e0_f64) * t1550 * t43692 - F::cast_from(0.11974241701863808564e0_f64) * t2604 * t9437 + F::cast_from(0.5987120850931904282e-1_f64) * t40690 + F::cast_from(0.39914139006212695214e-1_f64) * t1356 * t43698 + F::cast_from(0.212822999466489197e-4_f64) * t40695 + F::cast_from(0.1702583995731913576e-4_f64) * t40700 - F::cast_from(0.5107751987195740728e-4_f64) * t40703 - F::cast_from(0.5107751987195740728e-4_f64) * t40706 + F::cast_from(0.79828278012425390428e-1_f64) * t5928 * t8265 - F::cast_from(0.11918087970123395032e-3_f64) * t36448;
    (t43685, t43692, t43698, t43708)
}
