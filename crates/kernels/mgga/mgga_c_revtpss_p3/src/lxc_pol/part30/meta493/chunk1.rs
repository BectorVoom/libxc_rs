//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1845/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1845<F: Float>(t26079: F, t26080: F, t213: F, t7274: F, t1445: F, t2027: F, t25921: F, t25961: F, t25966: F, t26036: F, t26040: F, t26043: F, t26046: F, t26051: F, t26055: F, t26058: F, t26062: F, t26065: F, t26067: F, t26071: F, t26073: F, t26075: F, t4078: F, t561: F, t7279: F, t7295: F, t7304: F) -> (F, F, F) {
    let t26081 = t26079 * t26080;
    let t26084 = t213 * t7274;
    let t26087 = F::cast_from(0.13170898365871023197e1_f64) * t7279 * t4078 + F::cast_from(0.8673628188205199462e0_f64) * t7295 * t25961 + F::cast_from(0.4336814094102599731e0_f64) * t7295 * t25966 - F::cast_from(0.4336814094102599731e0_f64) * t2027 * t26036 - t26040 + t26043 + F::cast_from(0.4336814094102599731e0_f64) * t7295 * t26046 + F::cast_from(0.14456046980341999104e-1_f64) * t26051 - F::cast_from(0.19514881078765566038e-1_f64) * t26055 - t26058 + F::cast_from(0.8673628188205199462e0_f64) * t25921 * t7304 + F::cast_from(0.10975748638225852664e-1_f64) * t26062 + F::cast_from(0.19514881078765566038e-1_f64) * t26065 - F::cast_from(0.25702851531048074406e-1_f64) * t26067 - t26071 + F::cast_from(0.14456046980341999104e-1_f64) * t26073 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t26075 * t561 - F::cast_from(0.8673628188205199462e0_f64) * t7295 * t26081 - F::cast_from(0.13170898365871023197e1_f64) * t26084 * t1445;
    (t26081, t26084, t26087)
}
