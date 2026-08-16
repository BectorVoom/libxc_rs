//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3408/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3408(t16612: f64, t19137: f64, t3329: f64, t3333: f64, t5023: f64, t5024: f64, t63906: f64, t63907: f64, t63916: f64, t63918: f64, t63920: f64, t63923: f64, t63925: f64, t63927: f64, t63929: f64, t63934: f64, t63937: f64) -> f64 {
    let t63938 = -2.0_f64 * t16612 * t5023 * t5024 + 2.0_f64 * t19137 * t3329 * t5023 + 2.0_f64 * t3333 * t5023 * t63907 - t63906 - t63916 - t63918 - t63920 + t63923 - t63925 - t63927 + t63929 - t63934 + t63937;
    t63938
}
