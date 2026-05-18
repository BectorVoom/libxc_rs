//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 866/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk866<F: Float>(t2035: F, t27833: F, t7313: F, t7898: F, t1032: F, t1892: F, t1955: F, t1444: F, t7920: F, t25924: F, t1398: F, t543: F, t7910: F) -> (F, F, F, F, F, F) {
    let t27834 = t27833 * t2035;
    let t27835 = t7898 * t7313;
    let t27836 = t1892 * t1032;
    let t27837 = t1955 * t27836;
    let t27840 = t7920 * t1444;
    let t27841 = t25924 * t27840;
    let t27845 = t7910 * t1398 * t543;
    (t27834, t27835, t27836, t27837, t27841, t27845)
}
