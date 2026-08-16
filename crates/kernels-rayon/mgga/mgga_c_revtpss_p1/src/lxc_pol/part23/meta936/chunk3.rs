//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3079/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3079(t81379: f64, t81397: f64, t1132: f64, t1139: f64, t43771: f64, t44039: f64, t44040: f64, t68255: f64, t68257: f64, t81156: f64, t81158: f64, t81162: f64, t81167: f64) -> (f64, f64, f64) {
    let t81398 = t81379 + t81397;
    let t81399 = t1132 * t81398;
    let t81401 = t1139 * t81398;
    let t81403 = 0.39862222222222222223e0_f64 * t68255 - 0.26574814814814814815e0_f64 * t68257 - 0.2434271604938271605e0_f64 * t43771 + 0.19931111111111111111e0_f64 * t81156 - 0.59793333333333333333e0_f64 * t81158 + 0.99655555555555555554e0_f64 * t81162 + 0.39862222222222222223e1_f64 * t81167 + 0.1898925e1_f64 * t81399 + t44039 + t44040 + 0.3071625e0_f64 * t81401;
    (t81399, t81401, t81403)
}
