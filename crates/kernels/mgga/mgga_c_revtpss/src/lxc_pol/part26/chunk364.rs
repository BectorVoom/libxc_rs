//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 364/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk364<F: Float>(t550: F, t844: F, t247: F, t548: F, t235: F, t545: F) -> (F, F, F) {
    let t1404 = t844 * t550;
    let t1405 = t1404 * t247;
    let t1407 = F::cast_from(0.10003937560882938627e-2_f64) * t548 * t1405;
    let t1408 = t545 * t235;
    (t1405, t1407, t1408)
}
