//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1889/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1889(t26079: f64, t26080: f64, t213: f64, t7274: f64, t1445: f64, t2027: f64, t25921: f64, t25961: f64, t25966: f64, t26036: f64, t26040: f64, t26043: f64, t26046: f64, t26051: f64, t26055: f64, t26058: f64, t26062: f64, t26065: f64, t26067: f64, t26071: f64, t26073: f64, t26075: f64, t4078: f64, t561: f64, t7279: f64, t7295: f64, t7304: f64) -> (f64, f64, f64) {
    let t26081 = t26079 * t26080;
    let t26084 = t213 * t7274;
    let t26087 = 0.13170898365871023197e1_f64 * t7279 * t4078 + 0.8673628188205199462e0_f64 * t7295 * t25961 + 0.4336814094102599731e0_f64 * t7295 * t25966 - 0.4336814094102599731e0_f64 * t2027 * t26036 - t26040 + t26043 + 0.4336814094102599731e0_f64 * t7295 * t26046 + 0.14456046980341999104e-1_f64 * t26051 - 0.19514881078765566038e-1_f64 * t26055 - t26058 + 0.8673628188205199462e0_f64 * t25921 * t7304 + 0.10975748638225852664e-1_f64 * t26062 + 0.19514881078765566038e-1_f64 * t26065 - 0.25702851531048074406e-1_f64 * t26067 - t26071 + 0.14456046980341999104e-1_f64 * t26073 + 0.65854491829355115987e0_f64 * t213 * t26075 * t561 - 0.8673628188205199462e0_f64 * t7295 * t26081 - 0.13170898365871023197e1_f64 * t26084 * t1445;
    (t26081, t26084, t26087)
}
