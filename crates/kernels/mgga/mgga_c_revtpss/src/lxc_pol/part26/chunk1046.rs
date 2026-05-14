//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1046/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1046<F: Float>(t93302: F, t95854: F, t25310: F, t26544: F, t7064: F, t95575: F, t2067: F, t41117: F, t213: F, t225: F, t25383: F, t25391: F, t25394: F, t257: F, t26473: F, t26550: F, t26568: F, t7070: F, t7071: F, t7420: F, t886: F, t93126: F, t93130: F, t95715: F, t95823: F, t95825: F, t95832: F, t95834: F, t95836: F, t95847: F) -> (F,) {
    let t95855 = t93302 * t95854;
    let t95857 = t25310 * t26544;
    let t95859 = t7064 * t95575;
    let t95862 = 0.81814717454467823679e-4 * t41117 * t2067;
    let t95863 = 0.86736281882051994623e-1 * t95823 - 0.52041769129231196772e1 * t25391 * t95825 * t25394 - 0.26020884564615598386e1 * t25391 * t26550 * t93130 - 0.86736281882051994623e-1 * t95832 + 0.38554277296572111609e-1 * t95834 - 0.51405703062096148814e-2 * t95836 + 0.26020884564615598386e1 * t7070 * t7071 * t26473 * t886 + 0.13010442282307799193e1 * t93126 * t7420 + 0.26020884564615598386e1 * t25383 * t26568 + 0.32927245914677557992e-1 * t95847 + 0.65854491829355115987e0 * t213 * t95715 * t225 * t257 - 0.77108554593144223218e-1 * t95855 + 0.43368140941025997312e-1 * t95857 + 0.51405703062096148812e-1 * t95859 - t95862;
    (t95863,)
}
