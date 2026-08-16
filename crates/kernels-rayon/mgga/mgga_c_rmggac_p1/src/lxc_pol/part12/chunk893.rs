//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 893/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk893(t3369: f64, t34975: f64, t559: f64, t7455: f64, t2318: f64, t35039: f64, t7461: f64, t5016: f64, t9000: f64, t16043: f64, t8812: f64, t2320: f64, t35146: f64) -> (f64, f64, f64, f64, f64) {
    let t39445 = t34975 * t3369 * t559 * t7455;
    let t39449 = t34975 * t35039 * t2318 * t7461;
    let t39451 = t5016 * t9000;
    let t39452 = 0.15965655602485078085e0_f64 * t39451;
    let t39453 = t16043 * t8812;
    let t39455 = t35146 * t2320;
    (t39445, t39449, t39452, t39453, t39455)
}
