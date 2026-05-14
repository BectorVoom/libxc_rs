//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1073/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1073<F: Float>(t30055: F, t545: F, t2028: F, t1904: F, t2027: F, t25893: F, t25919: F, t25941: F, t25948: F, t25955: F, t27837: F, t27861: F, t27874: F, t27876: F, t27885: F, t27889: F, t27891: F, t27900: F, t27909: F, t30017: F, t30021: F, t30032: F, t6919: F, t7279: F, t7295: F, t7921: F) -> (F, F, F) {
    let t30056 = t545 * t30055;
    let t30057 = t2028 * t30056;
    let t30066 = -0.26020884564615598386e1 * t7295 * t30017 + 0.17347256376410398924e1 * t7295 * t30021 + 0.17347256376410398924e1 * t27837 * t7921 - 0.13170898365871023197e1 * t27909 * t1904 - 0.19514881078765566038e-1 * t27861 + t25893 - 0.28912093960683998208e-1 * t27874 + 0.51405703062096148812e-1 * t27876 + 0.8673628188205199462e0 * t7295 * t30032 - t25919 - 0.4336814094102599731e0 * t2027 * t30057 - 0.25702851531048074406e-1 * t27885 + 0.14456046980341999104e-1 * t27889 - 0.25702851531048074406e-1 * t27891 + 0.14456046980341999104e-1 * t27900 - t25941 + t25948 - 0.65854491829355115987e0 * t7279 * t6919 + t25955;
    (t30056, t30057, t30066)
}
