//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2679/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2679<F: Float>(t1668: F, t372: F, t4823: F, t1043: F, t11249: F, t11866: F, t19976: F, t19907: F, t3241: F, t1011: F, t6288: F, t697: F) -> (F, F, F, F, F) {
    let t66689 = t372 * t4823 * t1668;
    let t66702 = t11249 * t1043;
    let t66712 = t11866 * t19976;
    let t66714 = t3241 * t19907;
    let t66721 = t1011 * t697 * t6288;
    (t66689, t66702, t66712, t66714, t66721)
}
