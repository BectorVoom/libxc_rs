//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1063/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1063<F: Float>(t225: F, t29636: F, t1949: F, t6048: F, t25317: F, t6071: F, t7071: F, t233: F, t1957: F, t1558: F, t231: F, t7759: F, t7076: F, t1580: F, t1956: F, t213: F, t25303: F, t25307: F, t257: F, t27187: F, t27189: F, t27192: F, t27196: F, t27199: F, t27203: F, t27214: F, t27217: F, t29611: F, t6049: F, t6072: F, t7053: F, t7070: F, t7766: F, t7770: F, t7779: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t29637 = t29636 * t225;
    let t29643 = t1949 * t6048;
    let t29644 = t25317 * t29643;
    let t29654 = t1949 * t6071;
    let t29655 = t7071 * t29654;
    let t29658 = t233 * t29636;
    let t29659 = t1957 * t29658;
    let t29668 = t7759 * t1558 * t231;
    let t29669 = t7076 * t29668;
    let t29672 = 0.17347256376410398924e1 * t7070 * t29611 + 0.17347256376410398924e1 * t27199 * t7770 + 0.65854491829355115987e0 * t213 * t29637 * t257 - 0.13170898365871023197e1 * t27189 * t1580 - 0.26020884564615598386e1 * t7070 * t29644 - 0.65854491829355115987e0 * t7053 * t6072 + 0.25702851531048074406e-1 * t27187 - 0.8673628188205199462e0 * t7766 * t7779 + 0.13170898365871023197e1 * t7053 * t6049 + 0.8673628188205199462e0 * t7070 * t29655 - 0.4336814094102599731e0 * t1956 * t29659 - 0.14456046980341999104e-1 * t27192 - 0.10975748638225852664e-1 * t27196 + 0.19514881078765566038e-1 * t27203 + 0.14456046980341999104e-1 * t27214 - 0.25702851531048074406e-1 * t27217 + t25303 - t25307 + 0.8673628188205199462e0 * t7070 * t29669;
    (t29637, t29643, t29644, t29654, t29655, t29658, t29659, t29668, t29669, t29672)
}
