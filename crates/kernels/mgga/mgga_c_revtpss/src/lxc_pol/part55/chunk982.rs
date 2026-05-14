//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 982/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk982<F: Float>(t34216: F, t34240: F, t532: F, t1450: F, t2014: F, t1937: F, t28653: F, t1518: F, t2051: F) -> (F, F, F, F, F, F) {
    let t34241 = t34216 + t34240;
    let t34242 = t532 * t34241;
    let t34243 = t34242 * t1450;
    let t34244 = t2014 * t34243;
    let t34250 = 2.0 * t28653 * t1937;
    let t34251 = t2051 * t1518;
    (t34241, t34242, t34243, t34244, t34250, t34251)
}
