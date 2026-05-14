//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 826/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk826<F: Float>(t2594: F, t9094: F, t5218: F, t2568: F, t9016: F, t2576: F, t9079: F, t28324: F, t5290: F, t5289: F, t2587: F, t9062: F, t28303: F, t7303: F, t28749: F, t7430: F) -> (F, F, F, F, F, F, F) {
    let t29360 = t2594 * t9094;
    let t29362 = 6.0 * t5218 * t29360;
    let t29363 = t9016 * t2568;
    let t29365 = t2576 * t9079;
    let t29367 = t5290 * t28324;
    let t29368 = t5289 * t29367;
    let t29370 = t9062 * t2587;
    let t29372 = t7303 * t28303;
    let t29373 = t5289 * t29372;
    let t29375 = t7430 * t28749;
    (t29362, t29363, t29365, t29368, t29370, t29373, t29375)
}
