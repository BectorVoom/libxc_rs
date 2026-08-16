//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 827/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk827(t120: f64, t1631: f64, t16832: f64, t16835: f64, t16839: f64, t16842: f64, t16845: f64, t16849: f64, t16853: f64, t16855: f64, t16860: f64, t16864: f64, t16867: f64, t16870: f64, t16875: f64, t16878: f64, t2014: f64, t2021: f64, t3359: f64, t3360: f64, t534: f64, t7914: f64, t8948: f64, t8963: f64, t8977: f64, t8994: f64) -> f64 {
    let t16887 = 0.17557713923258613e0_f64 * t16832 * t3360 - 0.23410285231011484e0_f64 * t3359 * t16835 + 0.33205381699090447729e-3_f64 * t8948 * t16839 - 0.11705142615505742e0_f64 * t16842 * t3360 + 0.23410285231011484e0_f64 * t16845 * t120 - 0.26564305359272358183e-2_f64 * t2014 * t16849 - t16853 - 0.8854768453090786061e-3_f64 * t8963 * t16855 - 0.53128610718544716366e-2_f64 * t2014 * t16860 + 0.72343824494974941953e-3_f64 * t8963 * t16864 - 0.5116527820486904976e-1_f64 * t8977 * t16867 + 0.639565977560863122e-1_f64 * t2021 * t16870 - 0.25159457085530922489e-1_f64 * t7914 * t16867 + 0.319782988780431561e-1_f64 * t2021 * t16875 - 0.532971647967385935e-1_f64 * t534 * t16878 + 0.13977476158628290272e-1_f64 * t1631 * t16875 + 0.27954952317256580544e-1_f64 * t1631 * t16870 - 0.91641760171536135284e-3_f64 * t8994 * t16867;
    t16887
}
