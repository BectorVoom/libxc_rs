//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1851/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1851<F: Float>(t1936: F, t3813: F, t651: F, t4254: F, t7003: F, t1310: F, t7002: F, t2033: F, t530: F, t1450: F, t3829: F, t2014: F) -> (F, F, F, F, F, F, F, F) {
    let t25856 = t3813 * t1936;
    let t25858 = F::cast_from(2.0_f64) * t651 * t25856;
    let t25860 = F::cast_from(4.0_f64) * t4254 * t7003;
    let t25861 = t1310 * t7002;
    let t25863 = F::cast_from(4.0_f64) * t651 * t25861;
    let t25864 = t530 * t2033;
    let t25865 = t1450 * t3829;
    let t25866 = t25864 * t25865;
    let t25868 = F::cast_from(6.0_f64) * t2014 * t25866;
    (t25856, t25858, t25860, t25861, t25863, t25865, t25866, t25868)
}
