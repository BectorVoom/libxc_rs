//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 794/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk794(t3374: f64, t383: f64, t1005: f64, t1655: f64, t11204: f64, t126: f64, t120: f64, t12223: f64, t12225: f64, t12228: f64, t12464: f64, t12472: f64, t12479: f64, t12483: f64, t12489: f64, t12492: f64, t1631: f64, t2014: f64, t2015: f64, t2021: f64, t3359: f64, t3360: f64, t3368: f64, t534: f64, t7914: f64, t8942: f64, t8948: f64, t8959: f64, t8963: f64, t8977: f64, t8994: f64) -> f64 {
    let t12495 = t3374 * t383;
    let t12500 = t1005 * t1655;
    let t12503 = t11204 * t126;
    let t12512 = 0.17557713923258613e0_f64 * t12225 * t2015 - 0.23410285231011484e0_f64 * t12228 * t3360 + 0.33205381699090447729e-3_f64 * t8948 * t12464 - 0.11705142615505742e0_f64 * t3359 * t8942 + 0.23410285231011484e0_f64 * t12223 * t120 - 0.53128610718544716366e-2_f64 * t2014 * t12472 - 0.8854768453090786061e-3_f64 * t8959 * t3368 - 0.8854768453090786061e-3_f64 * t8963 * t12479 - 0.26564305359272358183e-2_f64 * t2014 * t12483 + 0.72343824494974941953e-3_f64 * t8963 * t12489 - 0.5116527820486904976e-1_f64 * t8977 * t12492 + 0.639565977560863122e-1_f64 * t2021 * t12495 - 0.25159457085530922489e-1_f64 * t7914 * t12492 + 0.319782988780431561e-1_f64 * t2021 * t12500 - 0.532971647967385935e-1_f64 * t534 * t12503 + 0.27954952317256580544e-1_f64 * t1631 * t12495 + 0.13977476158628290272e-1_f64 * t1631 * t12500 - 0.91641760171536135284e-3_f64 * t8994 * t12492;
    t12512
}
