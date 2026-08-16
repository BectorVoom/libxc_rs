//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1530/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1530<F: Float>(t1045: F, t2853: F, t999: F, t11774: F, t127: F, t3096: F, t3128: F, t11670: F, t11772: F, t3114: F, t11773: F, t11926: F) -> (F, F, F, F) {
    let t43057 = t1045 * t2853 * t999;
    let t43063 = t11774 * t127 * t3128 * t3096;
    let t43065 = t11670 * t11772;
    let t43066 = t3114 * t43065;
    let t43069 = t11926 * t11773;
    (t43057, t43063, t43066, t43069)
}
