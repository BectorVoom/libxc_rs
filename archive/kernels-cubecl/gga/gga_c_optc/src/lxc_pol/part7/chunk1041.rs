//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1041/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1041<F: Float>(t22668: F, t6742: F, t6745: F, t22531: F, t580: F, t587: F, t601: F, t1963: F, t2042: F, t22417: F, t22434: F, t22439: F, t22655: F, t22657: F, t22659: F, t22661: F, t22663: F, t22666: F) -> (F, F, F, F, F) {
    let t22669 = F::cast_from(0.65061485296689145287e-1_f64) * t22668;
    let t22670 = t6742 * t6745;
    let t22671 = F::cast_from(0.13012297059337829057e0_f64) * t22670;
    let t22675 = F::cast_from(0.58482233974552040708e0_f64) * t601 * t580 * t22531 * t587;
    let t22676 = t2042 * t1963;
    let t22677 = F::cast_from(120.0_f64) * t22676;
    let t22678 = -t22655 - t22417 + t22434 - t22439 + t22657 - t22659 + t22661 - t22663 + t22666 + t22669 - t22671 - t22675 + t22677;
    (t22669, t22671, t22675, t22677, t22678)
}
