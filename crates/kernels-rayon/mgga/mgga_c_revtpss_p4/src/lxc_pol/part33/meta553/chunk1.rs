//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1941/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1941(t29668: f64, t7076: f64, t1580: f64, t1956: f64, t213: f64, t25303: f64, t25307: f64, t257: f64, t27187: f64, t27189: f64, t27192: f64, t27196: f64, t27199: f64, t27203: f64, t27214: f64, t27217: f64, t29611: f64, t29637: f64, t29644: f64, t29655: f64, t29659: f64, t6049: f64, t6072: f64, t7053: f64, t7070: f64, t7766: f64, t7770: f64, t7779: f64) -> (f64, f64) {
    let t29669 = t7076 * t29668;
    let t29672 = 0.17347256376410398924e1_f64 * t7070 * t29611 + 0.17347256376410398924e1_f64 * t27199 * t7770 + 0.65854491829355115987e0_f64 * t213 * t29637 * t257 - 0.13170898365871023197e1_f64 * t27189 * t1580 - 0.26020884564615598386e1_f64 * t7070 * t29644 - 0.65854491829355115987e0_f64 * t7053 * t6072 + 0.25702851531048074406e-1_f64 * t27187 - 0.8673628188205199462e0_f64 * t7766 * t7779 + 0.13170898365871023197e1_f64 * t7053 * t6049 + 0.8673628188205199462e0_f64 * t7070 * t29655 - 0.4336814094102599731e0_f64 * t1956 * t29659 - 0.14456046980341999104e-1_f64 * t27192 - 0.10975748638225852664e-1_f64 * t27196 + 0.19514881078765566038e-1_f64 * t27203 + 0.14456046980341999104e-1_f64 * t27214 - 0.25702851531048074406e-1_f64 * t27217 + t25303 - t25307 + 0.8673628188205199462e0_f64 * t7070 * t29669;
    (t29669, t29672)
}
