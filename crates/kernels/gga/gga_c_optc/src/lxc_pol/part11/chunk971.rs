//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 971/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk971<F: Float>(t1313: F, t193: F, t21989: F, t3305: F, t6838: F, t2204: F, t3546: F, t2042: F, t3399: F, t2048: F, t2045: F, t544: F, t9521: F, t740: F, t9534: F, t3386: F, t6821: F) -> (F, F, F, F, F, F, F, F, F) {
    let t28366 = t193 * t21989 * t1313;
    let t28489 = t3305 * t6838;
    let t28530 = t3546 * t2204;
    let t28540 = t2042 * t3399;
    let t28552 = t2048 * t3399;
    let t28559 = t2045 * t3399;
    let t28561 = t544 * t9521;
    let t28610 = t9534 * t740;
    let t28617 = t3386 * t6821;
    (t28366, t28489, t28530, t28540, t28552, t28559, t28561, t28610, t28617)
}
