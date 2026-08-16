//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2154/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2154(t10778: f64, t1941: f64, t50538: f64, t93016: f64, t25222: f64, t4435: f64, t14868: f64, t2661: f64, t93082: f64, t14751: f64, t7045: f64, t14757: f64, t25234: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t99062 = t1941 * t10778;
    let t99063 = t99062 * t50538;
    let t99065 = 0.18071592998981862717e-4_f64 * t93016;
    let t99066 = t25222 * t4435;
    let t99069 = t2661 * t93082 * t14868;
    let t99070 = 0.57165357490759649296e-4_f64 * t99069;
    let t99071 = t7045 * t14751;
    let t99073 = t25234 * t14757;
    (t99063, t99065, t99066, t99070, t99071, t99073)
}
