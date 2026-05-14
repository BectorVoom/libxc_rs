//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 791/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk791<F: Float>(t28619: F, t28642: F, t28671: F, t28694: F, t16088: F, t16090: F, t1809: F, t2399: F, t28579: F, t28582: F, t28585: F, t28588: F, t28592: F, t28595: F, t28598: F, t5089: F, t604: F, t674: F, t8662: F) -> (F, F) {
    let t28696 = t28619 + t28642 + t28671 + t28694;
    let t28698 = -0.28111840756657074597e-1 * t5089 * t28579 + 0.14055920378328537299e-1 * t5089 * t28582 + 0.14055920378328537299e-1 * t1809 * t28585 + 0.14055920378328537299e-1 * t1809 * t28588 - 0.56223681513314149196e-1 * t674 * t28592 + 0.42167761134985611897e-1 * t674 * t28595 - 0.42167761134985611897e-1 * t1809 * t28598 - 0.14055920378328537299e-1 * t16088 - 0.28111840756657074597e-1 * t16090 - 3.0 * t2399 * t8662 - t604 * t28696;
    (t28696, t28698)
}
