//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1006/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1006<F: Float>(t240: F, t849: F, t14648: F, t775: F, t2661: F, t2652: F, t4345: F, t10716: F, t4349: F, t2689: F, t4372: F, t4354: F, t9775: F, t221: F, t2675: F, t4343: F) -> (F, F, F, F, F, F) {
    let t14832 = t849 * t240;
    let t14833 = t14648 * t775;
    let t14834 = t14832 * t14833;
    let t14836 = 0.28582678745379824648e-3 * t2661 * t14834;
    let t14837 = t2652 * t4345;
    let t14839 = t10716 * t4349;
    let t14846 = t2689 * t4372;
    let t14850 = t9775 * t4354;
    let t14857 = t2675 * t221 * t4343;
    (t14836, t14837, t14839, t14846, t14850, t14857)
}
