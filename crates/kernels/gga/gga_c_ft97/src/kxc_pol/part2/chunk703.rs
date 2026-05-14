//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 703/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk703<F: Float>(t120: F, t1595: F, t528: F, t12486: F, t1005: F, t3374: F, t383: F, t1655: F, t11204: F, t126: F, t12223: F, t12225: F, t12228: F, t12464: F, t12472: F, t12479: F, t12483: F, t1631: F, t2014: F, t2015: F, t2021: F, t3359: F, t3360: F, t3368: F, t534: F, t7914: F, t8942: F, t8948: F, t8959: F, t8963: F, t8977: F, t8994: F) -> (F,) {
    let t12488 = t1595 * t528 * t120;
    let t12489 = t12486 * t12488;
    let t12492 = t1005 * t1595;
    let t12495 = t3374 * t383;
    let t12500 = t1005 * t1655;
    let t12503 = t11204 * t126;
    let t12512 = 0.17557713923258613e0 * t12225 * t2015 - 0.23410285231011484e0 * t12228 * t3360 + 0.33205381699090447729e-3 * t8948 * t12464 - 0.11705142615505742e0 * t3359 * t8942 + 0.23410285231011484e0 * t12223 * t120 - 0.53128610718544716366e-2 * t2014 * t12472 - 0.8854768453090786061e-3 * t8959 * t3368 - 0.8854768453090786061e-3 * t8963 * t12479 - 0.26564305359272358183e-2 * t2014 * t12483 + 0.72343824494974941953e-3 * t8963 * t12489 - 0.5116527820486904976e-1 * t8977 * t12492 + 0.639565977560863122e-1 * t2021 * t12495 - 0.25159457085530922489e-1 * t7914 * t12492 + 0.319782988780431561e-1 * t2021 * t12500 - 0.532971647967385935e-1 * t534 * t12503 + 0.27954952317256580544e-1 * t1631 * t12495 + 0.13977476158628290272e-1 * t1631 * t12500 - 0.91641760171536135284e-3 * t8994 * t12492;
    (t12512,)
}
