//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 746/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk746<F: Float>(t2640: F, t2668: F, t2675: F, t2678: F, t2680: F, t7417: F, t7421: F, t7424: F, t7427: F, t7430: F, t7436: F, t7439: F, t7441: F, t7443: F, t7447: F, t7449: F, t7453: F, t7457: F, t7461: F, t7464: F, t7472: F, t878: F) -> (F,) {
    let t7474 = -0.23666877659387696117e-1 * t7417 + 0.35973654042269298099e1 * t7421 * t878 - 0.75734008510040627576e0 * t7424 - 0.1465164556873572827e3 * t7427 * t2675 + 0.73258227843678641352e2 * t7430 * t2680 + 0.18314556960919660338e2 * t7436 - 0.91572784804598301689e1 * t7439 + 11.0 / 108.0 * t7441 + t7443 / 54.0 + t7447 - 0.91572784804598301689e1 * t7449 * t7453 + 0.27471835441379490507e2 * t2668 * t7457 + 0.71000632978163088351e-1 * t2640 * t7461 - 0.13735917720689745254e2 * t2678 * t7464 + 0.94667510637550784468e-1 * t7472;
    (t7474,)
}
