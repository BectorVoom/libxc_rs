//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1106/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1106(t1083: f64, t1899: f64, t20716: f64, t1088: f64, t5870: f64, t1100: f64, t5490: f64, t1898: f64, t2743: f64, t237: f64, t5845: f64, t307: f64, t6000: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21184 = t1899 * t1083;
    let t21191 = 0.71233333333333333332e-1_f64 * t20716;
    let t21203 = t1088 * t5870;
    let t21212 = t1100 * t5490;
    let t21221 = t2743 * t1898;
    let t21267 = t237 * t5845;
    let t21346 = t307 * t6000;
    (t21184, t21191, t21203, t21212, t21221, t21267, t21346)
}
