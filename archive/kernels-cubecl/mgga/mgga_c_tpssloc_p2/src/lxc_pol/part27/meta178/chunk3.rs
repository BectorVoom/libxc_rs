//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 930/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk930<F: Float>(t1315: F, t1341: F, t1354: F, t1363: F, t1369: F, t3733: F, t3762: F, t3763: F, t3766: F, t3770: F, t3774: F, t3778: F, t3781: F, t3783: F, t3790: F, t3795: F, t3800: F, t3803: F, t3809: F, t3853: F, t3858: F, t3864: F, t3867: F, t3872: F, t3876: F, t559: F) -> F {
    let t3879 = t3762 + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t3763 + t3733 * t3766 / F::cast_from(16.0_f64) - t1315 * t3770 / F::cast_from(48.0_f64) + t3774 * t559 / F::cast_from(3072.0_f64) - t3778 * t1354 / F::cast_from(1536.0_f64) - F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t3781 - t3783 * t1369 / F::cast_from(384.0_f64) + t3790 * t3795 / F::cast_from(1536.0_f64) + F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t3800 + t3803 * t3809 / F::cast_from(384.0_f64) - t1341 * t3853 / F::cast_from(3072.0_f64) - t1341 * t3858 / F::cast_from(3072.0_f64) + t3864 + F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t3867 + F::cast_from(5.0_f64) / F::cast_from(768.0_f64) * t1363 * t3872 - t1363 * t3876 / F::cast_from(768.0_f64);
    t3879
}
