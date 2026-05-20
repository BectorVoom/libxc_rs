//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 886/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk886<F: Float>(t2134: F, t3682: F, t1234: F, t7623: F, t1210: F, t8945: F, t487: F, t7642: F, t1269: F, t3140: F, t1276: F, t2148: F) -> (F, F, F, F, F) {
    let t26877 = t2134 * t3682 / F::new(432.0);
    let t26880 = t1234 * t7623;
    let t26889 = t1210 * t8945;
    let t26894 = t7642 * t487;
    let t26895 = t26894 * t8945;
    let t26916 = t1269 * t3140;
    let t26918 = t2148 * t26916 * t1276;
    (t26877, t26880, t26889, t26895, t26918)
}
