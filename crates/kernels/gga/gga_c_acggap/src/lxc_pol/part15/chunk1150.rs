//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1150/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1150<F: Float>(t15386: F, t31195: F, t39891: F, t13287: F, t2297: F, t5616: F, t30817: F, t9649: F, t2030: F, t507: F, t8816: F, t1488: F, t2060: F, t2317: F) -> (F, F, F, F, F) {
    let t39893 = t31195 * t15386 * t39891;
    let t39897 = t31195 * t13287 * t2297 * t5616;
    let t39899 = t30817 * t9649;
    let t39907 = t2030 * t507 * t8816;
    let t39910 = t2060 * t1488 * t2317;
    (t39893, t39897, t39899, t39907, t39910)
}
