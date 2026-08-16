//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 827/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk827<F: Float>(t120: F, t1631: F, t16832: F, t16835: F, t16839: F, t16842: F, t16845: F, t16849: F, t16853: F, t16855: F, t16860: F, t16864: F, t16867: F, t16870: F, t16875: F, t16878: F, t2014: F, t2021: F, t3359: F, t3360: F, t534: F, t7914: F, t8948: F, t8963: F, t8977: F, t8994: F) -> F {
    let t16887 = F::cast_from(0.17557713923258613e0_f64) * t16832 * t3360 - F::cast_from(0.23410285231011484e0_f64) * t3359 * t16835 + F::cast_from(0.33205381699090447729e-3_f64) * t8948 * t16839 - F::cast_from(0.11705142615505742e0_f64) * t16842 * t3360 + F::cast_from(0.23410285231011484e0_f64) * t16845 * t120 - F::cast_from(0.26564305359272358183e-2_f64) * t2014 * t16849 - t16853 - F::cast_from(0.8854768453090786061e-3_f64) * t8963 * t16855 - F::cast_from(0.53128610718544716366e-2_f64) * t2014 * t16860 + F::cast_from(0.72343824494974941953e-3_f64) * t8963 * t16864 - F::cast_from(0.5116527820486904976e-1_f64) * t8977 * t16867 + F::cast_from(0.639565977560863122e-1_f64) * t2021 * t16870 - F::cast_from(0.25159457085530922489e-1_f64) * t7914 * t16867 + F::cast_from(0.319782988780431561e-1_f64) * t2021 * t16875 - F::cast_from(0.532971647967385935e-1_f64) * t534 * t16878 + F::cast_from(0.13977476158628290272e-1_f64) * t1631 * t16875 + F::cast_from(0.27954952317256580544e-1_f64) * t1631 * t16870 - F::cast_from(0.91641760171536135284e-3_f64) * t8994 * t16867;
    t16887
}
