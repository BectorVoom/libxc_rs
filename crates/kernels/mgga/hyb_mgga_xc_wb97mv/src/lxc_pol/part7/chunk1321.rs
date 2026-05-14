//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1321/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1321<F: Float>(t11374: F, t2597: F, t4372: F, t7318: F, t11531: F, t2558: F, t4345: F, t7362: F, t11446: F, t11450: F, t11453: F, t11454: F, t11501: F, t11502: F, t11505: F, t23082: F, t23363: F, t23373: F, t2534: F, t2535: F, t2550: F, t2556: F, t2574: F, t2589: F, t2595: F, t27591: F, t3549: F, t3567: F, t3600: F, t4333: F, t4346: F, t4359: F, t7254: F, t7316: F, t7328: F, t7333: F, t7360: F, t9385: F, t9492: F, t9501: F, t9547: F, t975: F, t994: F) -> (F,) {
    let t32304 = t11374 * t2597;
    let t32311 = t4372 * t7318;
    let t32331 = t11531 * t2558;
    let t32338 = t4345 * t7362;
    let t32357 = -2.0 * t2534 * t4346 * t2550 + 0.34631718211362927518e2 * t2595 * t32304 * t994 + 0.17315859105681463759e2 * t2595 * t11446 * t2589 + 0.10254018858216406658e4 * t7316 * t32311 * t2574 + 0.69263436422725855036e2 * t7254 * t11450 + 0.34631718211362927518e2 * t2595 * t3600 * t9385 + 0.20508037716432813316e4 * t23363 * t11454 + 0.10254018858216406658e4 * t7316 * t11453 * t2589 + 0.91082604192152556044e5 * t23373 * t4359 * t23082 * t2574 + 0.64327917994770140268e2 * t7328 * t11502 + 0.64327917994770140268e2 * t2556 * t32331 * t975 + 0.32163958997385070134e2 * t2556 * t11501 * t2550 + 0.2069040516770936012e4 * t7360 * t32338 * t2535 + 0.12865583598954028054e3 * t7328 * t11505 + 0.64327917994770140268e2 * t2556 * t3567 * t9492 + 6.0 * t2556 * t4346 * t2535 - 24.0 * t7333 * t4333 * t2535 + 12.0 * t9501 * t9547 - 8.0 * t27591 * t3549;
    (t32357,)
}
