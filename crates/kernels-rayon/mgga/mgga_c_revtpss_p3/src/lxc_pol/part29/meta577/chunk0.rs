//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1926/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1926(t14693: f64, t25270: f64, t14927: f64, t27261: f64, t10778: f64, t1941: f64, t50538: f64, t25222: f64, t4435: f64, t14868: f64, t2661: f64, t93082: f64) -> (f64, f64, f64, f64, f64) {
    let t99054 = t25270 * t14693;
    let t99056 = t27261 * t14927;
    let t99062 = t1941 * t10778;
    let t99063 = t99062 * t50538;
    let t99066 = t25222 * t4435;
    let t99069 = t2661 * t93082 * t14868;
    (t99054, t99056, t99063, t99066, t99069)
}
