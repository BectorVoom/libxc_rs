//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1005/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1005(t115009: f64, t115027: f64, t121782: f64, t126989: f64, t128086: f64, t128097: f64, t128110: f64, t1649: f64, t1877: f64, t1914: f64, t23788: f64, t24191: f64, t2522: f64, t26563: f64, t26744: f64, t26756: f64, t28771: f64, t28774: f64, t28789: f64, t28792: f64, t28795: f64, t31434: f64, t33065: f64, t33466: f64, t33531: f64, t5966: f64, t7114: f64, t7649: f64, t7656: f64, t8566: f64, t89953: f64, t92319: f64) -> f64 {
    let t128278 = -3.0_f64 * t24191 * t126989 - 3.0_f64 * t26756 * t89953 * t128110 + t1877 * t33466 * t1649 - t1877 * t31434 * t28795 / 2.0_f64 - t1877 * t7114 * t5966 * t1914 / 2.0_f64 + 3.0_f64 * t2522 * t33466 * t7649 - t1877 * t26744 * t33065 - t1877 * t121782 * t7656 - 3.0_f64 * t92319 * t33531 - 3.0_f64 * t26563 * t23788 * t128097 - 3.0_f64 / 2.0_f64 * t24191 * t23788 * t128086 - t1877 * t31434 * t28792 + t1877 * t115027 * t28789 - 3.0_f64 * t115009 * t28771 + 3.0_f64 * t2522 * t8566 * t28774;
    t128278
}
