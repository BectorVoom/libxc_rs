//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 794/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk794<F: Float>(t3374: F, t383: F, t1005: F, t1655: F, t11204: F, t126: F, t120: F, t12223: F, t12225: F, t12228: F, t12464: F, t12472: F, t12479: F, t12483: F, t12489: F, t12492: F, t1631: F, t2014: F, t2015: F, t2021: F, t3359: F, t3360: F, t3368: F, t534: F, t7914: F, t8942: F, t8948: F, t8959: F, t8963: F, t8977: F, t8994: F) -> F {
    let t12495 = t3374 * t383;
    let t12500 = t1005 * t1655;
    let t12503 = t11204 * t126;
    let t12512 = F::new(0.17557713923258613e0) * t12225 * t2015 - F::new(0.23410285231011484e0) * t12228 * t3360 + F::new(0.33205381699090447729e-3) * t8948 * t12464 - F::new(0.11705142615505742e0) * t3359 * t8942 + F::new(0.23410285231011484e0) * t12223 * t120 - F::new(0.53128610718544716366e-2) * t2014 * t12472 - F::new(0.8854768453090786061e-3) * t8959 * t3368 - F::new(0.8854768453090786061e-3) * t8963 * t12479 - F::new(0.26564305359272358183e-2) * t2014 * t12483 + F::new(0.72343824494974941953e-3) * t8963 * t12489 - F::new(0.5116527820486904976e-1) * t8977 * t12492 + F::new(0.639565977560863122e-1) * t2021 * t12495 - F::new(0.25159457085530922489e-1) * t7914 * t12492 + F::new(0.319782988780431561e-1) * t2021 * t12500 - F::new(0.532971647967385935e-1) * t534 * t12503 + F::new(0.27954952317256580544e-1) * t1631 * t12495 + F::new(0.13977476158628290272e-1) * t1631 * t12500 - F::new(0.91641760171536135284e-3) * t8994 * t12492;
    t12512
}
