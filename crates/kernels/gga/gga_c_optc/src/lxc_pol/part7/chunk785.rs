//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 785/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk785<F: Float>(t241: F, t7752: F, t7831: F, t7204: F, t7320: F, t7666: F, t7675: F, t7678: F, t7684: F, t7688: F, t7691: F, t7694: F, t7698: F, t7726: F, t7632: F) -> (F, F) {
    let t7833 = t241 * (t7752 + t7831);
    let t7834 = t7666 + t7675 - t7678 - t7684 - t7320 + t7688 + t7691 - t7694 - t7698 + t7204 + t7833 - t7726;
    let t7835 = t7632 + t7834;
    (t7833, t7835)
}
