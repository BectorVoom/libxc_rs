//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 641/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk641<F: Float>(t3753: F, t3758: F, t3763: F, t3766: F, t3770: F, t3773: F, t3776: F, t3782: F, t3785: F, t3790: F, t3793: F, t1125: F, t2469: F, t338: F, t3565: F, t3856: F, t3858: F, t3861: F, t3874: F, t3879: F, t3883: F, t884: F) -> (F, F) {
    let t3897 = F::new(0.40481770833333333335e-4) * t3753 - F::new(0.69504740211613770836e-3) * t3758 - F::new(0.25301106770833333335e-5) * t3763 + F::new(0.43440462632258606772e-4) * t3766 - F::new(0.4637672555408563478e-4) * t3770 - F::new(0.67528199161846004231e-6) * t3773 + F::new(0.11594181388521408695e-4) * t3776 - F::new(0.24581606547037760419e-8) * t3782 + F::new(0.21102562238076876322e-7) * t3785 + F::new(0.66295654499063700024e-7) * t3790 - F::new(0.18115908419564701085e-6) * t3793;
    let t3899 = -F::new(2.0) * t1125 * t3565 + F::new(2.0) * t2469 * t3883 + t338 * t3879 - t3897 * t884 - t3856 + t3858 - t3861 + t3874;
    (t3897, t3899)
}
