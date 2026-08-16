//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1186/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1186(t26481: f64, t2724: f64, t676: f64, t93302: f64, t25310: f64, t26544: f64, t7064: f64, t95575: f64, t2067: f64, t41117: f64, t213: f64, t225: f64, t25383: f64, t25391: f64, t25394: f64, t257: f64, t26473: f64, t26550: f64, t26568: f64, t7070: f64, t7071: f64, t7420: f64, t886: f64, t93126: f64, t93130: f64, t95715: f64, t95823: f64, t95825: f64, t95832: f64, t95834: f64, t95836: f64, t95847: f64) -> (f64, f64) {
    let t95854 = t26481 * t676 * t2724;
    let t95855 = t93302 * t95854;
    let t95857 = t25310 * t26544;
    let t95859 = t7064 * t95575;
    let t95862 = 0.81814717454467823679e-4_f64 * t41117 * t2067;
    let t95863 = 0.86736281882051994623e-1_f64 * t95823 - 0.52041769129231196772e1_f64 * t25391 * t95825 * t25394 - 0.26020884564615598386e1_f64 * t25391 * t26550 * t93130 - 0.86736281882051994623e-1_f64 * t95832 + 0.38554277296572111609e-1_f64 * t95834 - 0.51405703062096148814e-2_f64 * t95836 + 0.26020884564615598386e1_f64 * t7070 * t7071 * t26473 * t886 + 0.13010442282307799193e1_f64 * t93126 * t7420 + 0.26020884564615598386e1_f64 * t25383 * t26568 + 0.32927245914677557992e-1_f64 * t95847 + 0.65854491829355115987e0_f64 * t213 * t95715 * t225 * t257 - 0.77108554593144223218e-1_f64 * t95855 + 0.43368140941025997312e-1_f64 * t95857 + 0.51405703062096148812e-1_f64 * t95859 - t95862;
    (t95854, t95863)
}
