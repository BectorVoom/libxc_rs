//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 317/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk317<F: Float>(t1445: F, t2572: F, t2581: F, t701: F, t773: F, t954: F, t1: F, t935: F, t106: F, t316: F, t769: F, t774: F, t959: F, t1457: F, t1966: F, t1991: F, t1998: F, t2004: F, t2009: F, t2103: F, t2598: F, t2601: F, t2605: F, t2608: F, t2613: F, t2619: F, t2621: F, t2625: F, t2629: F, t2631: F, t2635: F, t2638: F, t2639: F, t780: F, t807: F, t813: F, t833: F) -> (F, F, F, F) {
    let t2642 = t1445 * t2572;
    let t2645 = t2581 * t701;
    let t2646 = t1445 * t2645;
    let t2649 = t773 * t954;
    let t2652 = t935 * t1;
    let t2653 = t2652 * t106;
    let t2654 = t2653 * t316;
    let t2657 = t769 * t774;
    let t2658 = t2657 * t959;
    let t2660 = t1457 * t2572;
    let t2663 = 0.30674340763136599741e1 * t833 * t2598 + 0.14896037479937677779e-1 * t2601 - 0.14896037479937677779e-1 * t2605 + 0.12780975317973583226e0 * t2608 - 0.14896037479937677779e-1 * t2613 + 0.95857314884801874192e-1 * t2619 + 0.51123901271894332902e0 * t1991 * t2621 - 0.51123901271894332902e0 * t1966 * t2625 - 0.29792074959875355558e-1 * t2629 + 0.71500979903700853338e0 * t2103 * t2631 - 0.46011511144704899612e1 * t813 * t2635 - 0.10725146985555128001e1 * t2638 * t2639 + 0.23005755572352449806e1 * t807 * t2642 - 0.23005755572352449806e1 * t1998 * t2646 - 0.35750489951850426669e0 * t2649 * t2009 + 0.35750489951850426669e0 * t780 * t2654 + 0.14896037479937677779e-1 * t2658 + 0.35750489951850426669e0 * t2004 * t2660;
    (t2645, t2658, t2660, t2663)
}
