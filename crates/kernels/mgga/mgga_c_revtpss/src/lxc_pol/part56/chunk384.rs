//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 384/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk384<F: Float>(t1963: F, t30: F, t1940: F, t1962: F, t207: F, t198: F, t892: F, t33: F, t1312: F, t1936: F, t196: F, t511: F, t197: F) -> (F, F, F, F, F, F, F) {
    let t1964 = t1963 * t30;
    let t1966 = t1940 * t1964 / 2.0;
    let t1993 = t207 * t1962;
    let t1995 = t198 * t1993 * t892;
    let t2000 = t1963 * t33;
    let t2002 = t1940 * t2000 / 2.0;
    let t2010 = 2.0 * t1312 * t1936;
    let t2013 = t511 * t196;
    let t2014 = t2013 * t197;
    (t1966, t1993, t1995, t2002, t2010, t2013, t2014)
}
