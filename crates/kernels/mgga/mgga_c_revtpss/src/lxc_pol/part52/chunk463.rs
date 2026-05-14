//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 463/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk463<F: Float>(t2608: F, t520: F, t512: F, t19: F, t27: F, t521: F, t14: F, t22: F, t1320: F, t1333: F, t123: F, t2630: F, t1337: F, t2619: F, t514: F, t517: F) -> (F, F, F, F, F, F, F, F) {
    let t3853 = t520 * t2608;
    let t3854 = t512 * t3853;
    let t3857 = t19 * t27;
    let t3859 = 20.0 * t3857 * t521;
    let t3860 = t14 * t22;
    let t3862 = 12.0 * t3860 * t521;
    let t3867 = 8.0 * t1320 * t1333;
    let t3869 = t520 * t123;
    let t3871 = 0.10843581300301739842e-1 * t3869 * t2630;
    let t3873 = 0.24415263074675393405e-3 * t1337 * t2619;
    let t3874 = 1.0 / t514;
    let t3881 = 1.0 / t517;
    (t3854, t3859, t3862, t3867, t3871, t3873, t3874, t3881)
}
