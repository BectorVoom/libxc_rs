//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1278/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1278(t14143: f64, t14144: f64, t14141: f64, t10069: f64, t5737: f64, t5710: f64, t72: f64, t1432: f64, t686: f64, t136: f64, t1892: f64, t2457: f64, t3964: f64) -> (f64, f64, f64, f64) {
    let t14145 = t14143 * t14144;
    let t14146 = t14141 * t14145;
    let t14149 = t10069 * t5737;
    let t14155 = t5710 * t72;
    let t14158 = 0.19514881078765566038e-1_f64 * t1432 * t14155 * t686;
    let t14159 = t1892 * t136;
    let t14161 = t3964 * t14159 * t2457;
    (t14146, t14149, t14158, t14161)
}
