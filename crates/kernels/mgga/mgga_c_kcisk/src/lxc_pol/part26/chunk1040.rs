//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1040/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1040<F: Float>(t14743: F, t25778: F, t25864: F, t25879: F, t25881: F, t25884: F, t25887: F, t25890: F, t25893: F, t25897: F, t25900: F, t25904: F, t27643: F, t27661: F, t516: F, t1550: F, t240: F, t25669: F, t25672: F, t25781: F, t25840: F, t25871: F, t27577: F, t27605: F, t27641: F, t4486: F, t7825: F) -> (F,) {
    let t27665 = 0.1025389702100779493e4 * t14743 * t27643 + t25864 - 0.3109e-1 * t27661 * t516 - 0.19751789702565206229e-1 * t25778 + t25879 - t25881 - t25884 + t25887 + t25890 + t25893 - t25897 - t25900 - t25904;
    let t27681 = 0.11696446794910408142e1 * t1550 * t25871 - t25864 + t240 * (t27577 + t27605 + t27641 + t27665) + 0.19751789702565206229e-1 * t240 * t25778 + 0.23392893589820816284e1 * t1550 * t25781 - 0.58482233974552040708e0 * t1550 * t25840 - 0.17315755899375863299e2 * t4486 * t7825 - t25879 + t25881 + t25884 - t25887 - t25890 - t25893 + t25897 + t25900 + t25904 - 0.34631511798751726598e2 * t1550 * t25672 - 0.17315755899375863299e2 * t1550 * t25669;
    (t27681,)
}
