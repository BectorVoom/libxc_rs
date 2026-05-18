//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 417/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk417<F: Float>(t1312: F, t1936: F, t1932: F, t196: F, t511: F, t197: F) -> (F, F, F) {
    let t2010 = F::new(2.0) * t1312 * t1936;
    let t2011 = t1932 + t2010;
    let t2013 = t511 * t196;
    let t2014 = t2013 * t197;
    (t2011, t2013, t2014)
}
