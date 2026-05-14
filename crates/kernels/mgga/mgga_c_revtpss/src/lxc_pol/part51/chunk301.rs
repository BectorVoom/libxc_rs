//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 301/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk301<F: Float>(t1405: F, t548: F, t235: F, t545: F, t239: F, t820: F, t530: F, t549: F) -> (F, F, F, F) {
    let t1407 = 0.10003937560882938627e-2 * t548 * t1405;
    let t1408 = t545 * t235;
    let t1410 = t820 * t1408 * t239;
    let t1412 = 1.0 / t549 / t530;
    (t1407, t1408, t1410, t1412)
}
