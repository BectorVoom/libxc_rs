//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1026/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1026<F: Float>(t11013: F, t17375: F, t17379: F, t17382: F, t17385: F, t23466: F, t23472: F, t23481: F, t23484: F, t23487: F, t23490: F, t23539: F, t23542: F, t23545: F, t23547: F, t23550: F, t23570: F, t23576: F, t23579: F, t23583: F, t23679: F, t23703: F) -> (F,) {
    let t23705 = -0.1898925e1 * t23539 - 0.9494625e0 * t23542 - 0.76790625e-1 * t23545 + 0.3071625e0 * t23547 + 0.15358125e0 * t23550 - 0.18257037037037037037e0 * t11013 - 0.43816888888888888888e0 * t17375 - 0.39862222222222222222e0 * t17379 - 0.26574814814814814815e0 * t17382 - 0.36514074074074074073e0 * t17385 + t23679 + 0.59793333333333333334e0 * t23487 - 0.19931111111111111111e0 * t23484 - 0.21908444444444444444e0 * t23570 - 0.19931111111111111111e0 * t23472 + 0.99655555555555555557e-1 * t23481 - 0.29896666666666666667e0 * t23490 + 0.142419375e1 * t23576 + 0.32862666666666666666e0 * t23579 + 0.11958666666666666667e1 * t23466 + 0.32862666666666666666e0 * t23583 + t23703;
    (t23705,)
}
