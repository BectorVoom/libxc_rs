//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1082/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1082<F: Float>(t24469: F, t24535: F, t2574: F, t2579: F, t7445: F, t854: F, t116: F, t23533: F, t286: F, t10926: F, t24494: F, t2668: F, t2586: F, t7851: F, t893: F, t10953: F, t155: F, t23662: F, t24478: F, t24492: F, t24496: F, t24498: F, t24504: F, t24507: F, t24510: F, t24516: F, t24519: F, t24522: F, t24530: F, t2674: F, t2675: F, t2678: F, t2679: F, t2680: F, t2813: F, t3625: F, t3835: F, t7360: F, t7449: F, t7451: F, t7491: F, t7983: F) -> (F, F) {
    let t24536 = t24535 * t24469;
    let t24540 = t2574 * t2579;
    let t24542 = t854 * t7445;
    let t24546 = 5.0 / 486.0 * t286 * t116 * t23533;
    let t24548 = t2668 * t24494 * t10926;
    let t24550 = t2586 * t7851;
    let t24551 = t893 * t24550;
    let t24553 = -0.36629113921839320676e2 * t7449 * t24478 * t155 * t7983 - 0.19318136643975017455e0 * t10953 * t7360 + 0.24147670804968771818e-1 * t24492 - 0.36629113921839320676e2 * t24496 + 0.36629113921839320676e2 * t2668 * t24498 * t3625 + 0.48838818562452427568e2 * t24504 + 0.18558751053731922476e4 * t24507 * t2675 - 0.9279375526865961238e3 * t24510 * t2680 + 0.6104852320306553446e1 * t24516 - 0.12209704640613106892e2 * t24519 - 0.13735917720689745254e2 * t2678 * t24522 * t2679 + 0.27471835441379490507e2 * t2668 * t24522 * t2674 - 0.28977204965962526181e-1 * t24530 + 0.65198711173415683908e-1 * t3835 * t2813 * t23662 - 0.73258227843678641351e2 * t7491 * t7451 * t24536 - 11.0 / 81.0 * t24540 - 10.0 / 243.0 * t24542 - t24546 + 0.73258227843678641352e2 * t24548 + 0.48295341609937543636e-2 * t24551;
    (t24550, t24553)
}
