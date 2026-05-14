//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1079/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1079<F: Float>(t25949: F, t786: F, t7286: F, t225: F, t26034: F, t1426: F, t3999: F, t26044: F, t4003: F, t213: F, t7274: F, t1445: F, t2027: F, t25921: F, t25961: F, t25966: F, t26036: F, t26040: F, t26043: F, t26046: F, t26051: F, t26055: F, t26058: F, t26062: F, t26065: F, t26067: F, t26071: F, t4078: F, t561: F, t7279: F, t7295: F, t7304: F) -> (F, F, F, F, F, F) {
    let t26072 = t786 * t25949;
    let t26073 = t26072 * t7286;
    let t26075 = t26034 * t225;
    let t26079 = t1426 * t3999;
    let t26080 = t26044 * t4003;
    let t26081 = t26079 * t26080;
    let t26084 = t213 * t7274;
    let t26087 = 0.13170898365871023197e1 * t7279 * t4078 + 0.8673628188205199462e0 * t7295 * t25961 + 0.4336814094102599731e0 * t7295 * t25966 - 0.4336814094102599731e0 * t2027 * t26036 - t26040 + t26043 + 0.4336814094102599731e0 * t7295 * t26046 + 0.14456046980341999104e-1 * t26051 - 0.19514881078765566038e-1 * t26055 - t26058 + 0.8673628188205199462e0 * t25921 * t7304 + 0.10975748638225852664e-1 * t26062 + 0.19514881078765566038e-1 * t26065 - 0.25702851531048074406e-1 * t26067 - t26071 + 0.14456046980341999104e-1 * t26073 + 0.65854491829355115987e0 * t213 * t26075 * t561 - 0.8673628188205199462e0 * t7295 * t26081 - 0.13170898365871023197e1 * t26084 * t1445;
    (t26072, t26075, t26079, t26081, t26084, t26087)
}
