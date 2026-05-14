//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 975/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk975<F: Float>(t1038: F, t20602: F, t3712: F, t1875: F, t2972: F, t134: F, t8957: F, t5549: F, t116: F, t126: F, t1602: F, t1908: F, t19509: F, t681: F, t5247: F, t9261: F) -> (F, F, F, F, F, F, F) {
    let t26836 = t3712 * t1038 * t20602;
    let t26887 = t1875 * t2972;
    let t26995 = t8957 * t134;
    let t26996 = t26995 * t5549;
    let t27036 = t116 * t126;
    let t27043 = t1908 * t681 * t1038 * t19509 * t1602;
    let t27063 = t5247 * t681 * t9261;
    (t26836, t26887, t26995, t26996, t27036, t27043, t27063)
}
