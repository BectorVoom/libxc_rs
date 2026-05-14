//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 636/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk636<F: Float>(t10568: F, t311: F, t3841: F, t579: F, t10585: F, t4726: F, t26: F, t10593: F, t1659: F, t10570: F, t10572: F, t10574: F, t10576: F, t10579: F, t10582: F, t10587: F, t10590: F, t10595: F, t10598: F) -> (F, F, F, F, F, F) {
    let t10639 = 0.93932222222222222223e0 * t10568;
    let t10641 = t311 * t3841 * t579;
    let t10642 = 0.36793333333333333333e0 * t10641;
    let t10643 = t4726 * t10585;
    let t10644 = t26 * t10643;
    let t10646 = t1659 * t10593;
    let t10647 = t26 * t10646;
    let t10649 = 28.0 / 27.0 * t10568;
    let t10660 = -t10649 - 4.0 / 9.0 * t10570 + 2.0 / 9.0 * t10572 - 2.0 / 3.0 * t10574 + t10576 / 3.0 - 10.0 / 27.0 * t10579 + 4.0 / 3.0 * t10582 - 2.0 / 3.0 * t10587 - 2.0 * t10590 + 2.0 * t10595 - t10598 / 3.0;
    (t10639, t10641, t10642, t10644, t10647, t10660)
}
