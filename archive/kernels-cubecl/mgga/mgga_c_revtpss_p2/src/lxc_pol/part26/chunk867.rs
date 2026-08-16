//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 867/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk867<F: Float>(t10493: F, t11082: F, t11092: F, t11093: F, t1100: F, t3333: F, t3335: F, t389: F, t2918: F, t936: F, t2874: F, t2926: F, t934: F) -> (F, F, F, F, F) {
    let t11095 = t10493 + t11082 + t11092 + t11093;
    let t11105 = t3333 * t1100;
    let t11108 = F::cast_from(1.0_f64) / t3335 / t389;
    let t11112 = t936 * t2918;
    let t11114 = F::cast_from(6.0_f64) * t2874 * t11112;
    let t11116 = t2918 * t2926 * t934;
    (t11095, t11105, t11108, t11114, t11116)
}
