//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 939/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk939(t29616: f64, t746: f64, t11927: f64, t2560: f64, t9058: f64, t2576: f64, t9066: f64, t7337: f64, t9043: f64, t29586: f64, t29589: f64, t29591: f64, t29595: f64, t29598: f64, t29601: f64, t29603: f64, t29607: f64, t29609: f64, t29611: f64, t29614: f64) -> (f64, f64, f64, f64, f64) {
    let t29617 = t746 * t29616;
    let t29618 = t11927 * t29617;
    let t29620 = t2560 * t9058;
    let t29622 = t2576 * t9066;
    let t29624 = t7337 * t9043;
    let t29625 = -t29586 / 256.0_f64 - t29589 + t29591 / 8.0_f64 + t29595 / 54.0_f64 + t29598 / 192.0_f64 - t29601 / 16.0_f64 + t29603 / 8.0_f64 - 3.0_f64 / 8.0_f64 * t29607 - t29609 / 8.0_f64 - t29611 / 64.0_f64 + t29614 / 9.0_f64 + 3.0_f64 / 128.0_f64 * t29618 - 2.0_f64 / 3.0_f64 * t29620 - t29622 / 192.0_f64 + t29624;
    (t29618, t29620, t29622, t29624, t29625)
}
