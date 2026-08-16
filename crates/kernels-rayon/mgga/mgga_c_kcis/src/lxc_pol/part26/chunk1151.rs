//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1151/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1151(t6284: f64, t7909: f64, t5709: f64, t27438: f64, t6281: f64, t5701: f64, t28356: f64, t8164: f64, t1394: f64, t5653: f64, t7923: f64, t2243: f64, t7193: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t29357 = t7909 * t6284;
    let t29358 = t5709 * t29357;
    let t29361 = t27438 * t6281;
    let t29362 = t5701 * t29361;
    let t29365 = t28356 * t8164;
    let t29366 = t1394 * t29365;
    let t29368 = t5653 * t6281;
    let t29369 = t7923 * t29368;
    let t29370 = t1394 * t29369;
    let t29372 = t7193 * t2243;
    (t29357, t29358, t29361, t29362, t29365, t29366, t29368, t29369, t29370, t29372)
}
