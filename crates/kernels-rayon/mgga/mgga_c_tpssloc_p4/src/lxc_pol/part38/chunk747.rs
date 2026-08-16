//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 747/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk747(t1369: f64, t3866: f64, t1995: f64, t241: f64, t67: f64, t3734: f64, t820: f64, t1367: f64, t3719: f64, t1315: f64, t1341: f64, t1354: f64, t1363: f64, t3733: f64, t3762: f64, t3763: f64, t3766: f64, t3770: f64, t3774: f64, t3778: f64, t3781: f64, t3783: f64, t3790: f64, t3795: f64, t3800: f64, t3803: f64, t3809: f64, t3853: f64, t3858: f64, t3864: f64, t559: f64) -> (f64, f64, f64, f64, f64) {
    let t3867 = t3866 * t1369;
    let t3869 = t241 * t1995;
    let t3870 = t3869 * t67;
    let t3872 = t3870 * t820 * t3734;
    let t3876 = t1367 * t820 * t3719;
    let t3879 = t3762 + 7.0_f64 / 72.0_f64 * t3763 + t3733 * t3766 / 16.0_f64 - t1315 * t3770 / 48.0_f64 + t3774 * t559 / 3072.0_f64 - t3778 * t1354 / 1536.0_f64 - 7.0_f64 / 2304.0_f64 * t3781 - t3783 * t1369 / 384.0_f64 + t3790 * t3795 / 1536.0_f64 + 7.0_f64 / 2304.0_f64 * t3800 + t3803 * t3809 / 384.0_f64 - t1341 * t3853 / 3072.0_f64 - t1341 * t3858 / 3072.0_f64 + t3864 + 7.0_f64 / 576.0_f64 * t3867 + 5.0_f64 / 768.0_f64 * t1363 * t3872 - t1363 * t3876 / 768.0_f64;
    (t3867, t3870, t3872, t3876, t3879)
}
