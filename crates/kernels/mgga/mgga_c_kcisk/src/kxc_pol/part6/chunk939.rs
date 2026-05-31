//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 939/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk939<F: Float>(t29616: F, t746: F, t11927: F, t2560: F, t9058: F, t2576: F, t9066: F, t7337: F, t9043: F, t29586: F, t29589: F, t29591: F, t29595: F, t29598: F, t29601: F, t29603: F, t29607: F, t29609: F, t29611: F, t29614: F) -> (F, F, F, F, F) {
    let t29617 = t746 * t29616;
    let t29618 = t11927 * t29617;
    let t29620 = t2560 * t9058;
    let t29622 = t2576 * t9066;
    let t29624 = t7337 * t9043;
    let t29625 = -t29586 / F::cast_from(256.0_f64) - t29589 + t29591 / F::cast_from(8.0_f64) + t29595 / F::cast_from(54.0_f64) + t29598 / F::cast_from(192.0_f64) - t29601 / F::cast_from(16.0_f64) + t29603 / F::cast_from(8.0_f64) - F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t29607 - t29609 / F::cast_from(8.0_f64) - t29611 / F::cast_from(64.0_f64) + t29614 / F::cast_from(9.0_f64) + F::cast_from(3.0_f64) / F::cast_from(128.0_f64) * t29618 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t29620 - t29622 / F::cast_from(192.0_f64) + t29624;
    (t29618, t29620, t29622, t29624, t29625)
}
