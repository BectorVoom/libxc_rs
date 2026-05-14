//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 937/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk937<F: Float>(t3143: F, t36865: F, t8514: F, t31927: F, t7150: F, t3268: F, t8513: F, t93488: F, t1078: F, t1976: F, t1982: F, t3140: F, t31966: F, t31970: F, t3057: F, t7165: F) -> (F, F, F, F, F, F) {
    let t120653 = t36865 * t3143;
    let t120654 = t8514 * t120653;
    let t120664 = t7150 * t31927;
    let t120671 = t8513 * t93488 * t3268;
    let t120676 = t1982 * t1976 * t3140 * t1078;
    let t120696 = t31966 * t31970;
    let t120702 = t3057 * t7165;
    (t120654, t120664, t120671, t120676, t120696, t120702)
}
