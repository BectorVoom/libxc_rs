//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1208/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1208(t27279: f64, t32478: f64, t1032: f64, t7997: f64, t1955: f64, t126250: f64, t8477: f64, t126210: f64, t119894: f64, t119913: f64, t121891: f64, t121896: f64, t121897: f64, t27300: f64, t27322: f64, t32434: f64, t32464: f64, t7079: f64) -> (f64, f64, f64) {
    let t127698 = t32478 * t27279;
    let t127703 = t7997 * t1032;
    let t127704 = t1955 * t127703;
    let t127707 = t8477 * t126250;
    let t127710 = 0.263521689745817692e-2_f64 * t126210;
    let t127711 = 0.17347256376410398924e1_f64 * t32434 * t27322 + t121891 + 0.14456046980341999104e-1_f64 * t127698 - 0.66934509195437693771e-4_f64 * t119894 + t121896 + t121897 - 0.52041769129231196772e1_f64 * t32434 * t27300 + 0.8673628188205199462e0_f64 * t127704 * t7079 - t119913 - 0.11423947533020470523e1_f64 * t127707 * t32464 + t127710;
    (t127703, t127704, t127711)
}
