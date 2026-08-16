//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 922/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk922(t29367: f64, t5289: f64, t2587: f64, t9062: f64, t28303: f64, t7303: f64, t28749: f64, t7430: f64, t7429: f64, t17969: f64, t9047: f64, t17933: f64, t9072: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29368 = t5289 * t29367;
    let t29370 = t9062 * t2587;
    let t29372 = t7303 * t28303;
    let t29373 = t5289 * t29372;
    let t29375 = t7430 * t28749;
    let t29376 = t7429 * t29375;
    let t29378 = t17969 * t9047;
    let t29380 = t17933 * t9072;
    (t29368, t29370, t29373, t29376, t29378, t29380)
}
