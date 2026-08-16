//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1106/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1106(t13717: f64, t13742: f64, t13772: f64, t13775: f64, t13777: f64, t13881: f64, t13886: f64, t13888: f64, t13892: f64, t13910: f64, t13912: f64, t13915: f64, t13918: f64, t13921: f64, t13924: f64, t13927: f64, t13931: f64, t13934: f64, t13951: f64, t9681: f64, t9683: f64, t9691: f64) -> f64 {
    let t13953 = 0.142419375e1_f64 * t13772 - 0.76790625e-1_f64 * t13881 - 0.1898925e1_f64 * t13775 - 0.9494625e0_f64 * t13777 + 0.3071625e0_f64 * t13886 + 0.15358125e0_f64 * t13888 - 0.16431333333333333333e0_f64 * t13892 + 0.99655555555555555557e-1_f64 * t9681 + 0.66437037037037037038e-1_f64 * t9683 - 0.26574814814814814816e0_f64 * t9691 + t13910 + 0.36514074074074074074e-1_f64 * t13912 - 0.27385555555555555556e-1_f64 * t13915 - 0.36514074074074074075e-1_f64 * t13918 - 0.10954222222222222222e0_f64 * t13921 + 0.16431333333333333333e0_f64 * t13924 + 0.65725333333333333332e0_f64 * t13927 + 0.21924222222222222222e1_f64 * t13717 + 0.16431333333333333333e0_f64 * t13931 - 0.49293999999999999999e0_f64 * t13934 - 0.59793333333333333334e0_f64 * t13742 + t13951;
    t13953
}
