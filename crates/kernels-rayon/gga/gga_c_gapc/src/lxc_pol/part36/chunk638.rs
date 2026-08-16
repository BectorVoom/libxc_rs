//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 638/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk638(t3753: f64, t3758: f64, t3763: f64, t3766: f64, t3770: f64, t3773: f64, t3776: f64, t3782: f64, t3785: f64, t3790: f64, t3793: f64, t1125: f64, t2469: f64, t338: f64, t3565: f64, t3856: f64, t3858: f64, t3861: f64, t3874: f64, t3879: f64, t3883: f64, t884: f64) -> (f64, f64) {
    let t3897 = 0.40481770833333333335e-4_f64 * t3753 - 0.69504740211613770836e-3_f64 * t3758 - 0.25301106770833333335e-5_f64 * t3763 + 0.43440462632258606772e-4_f64 * t3766 - 0.4637672555408563478e-4_f64 * t3770 - 0.67528199161846004231e-6_f64 * t3773 + 0.11594181388521408695e-4_f64 * t3776 - 0.24581606547037760419e-8_f64 * t3782 + 0.21102562238076876322e-7_f64 * t3785 + 0.66295654499063700024e-7_f64 * t3790 - 0.18115908419564701085e-6_f64 * t3793;
    let t3899 = -2.0_f64 * t1125 * t3565 + 2.0_f64 * t2469 * t3883 + t338 * t3879 - t3897 * t884 - t3856 + t3858 - t3861 + t3874;
    (t3897, t3899)
}
