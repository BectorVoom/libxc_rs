//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 699/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk699<F: Float>(t603: F, t6632: F, t6407: F, t6424: F, t6427: F, t601: F, t1986: F, t1998: F, t580: F, t587: F, t6419: F, t1994: F) -> (F, F, F, F, F, F, F) {
    let t6633 = t6632 * t603;
    let t6634 = F::cast_from(0.17544670192365612213e1_f64) * t6633;
    let t6636 = t6424 * t6407 * t6427;
    let t6638 = F::cast_from(0.1025389702100779493e4_f64) * t601 * t6636;
    let t6639 = t1986 * t1998;
    let t6640 = F::cast_from(0.17544670192365612213e1_f64) * t6639;
    let t6642 = t580 * t6419 * t587;
    let t6644 = F::cast_from(0.58482233974552040708e0_f64) * t601 * t6642;
    let t6646 = t1986 * t1994;
    (t6634, t6636, t6638, t6640, t6642, t6644, t6646)
}
