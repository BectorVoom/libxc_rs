//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 682/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk682(t167: f64, t2185: f64, t26909: f64, t1882: f64, t6710: f64, t6627: f64, t8392: f64, t3052: f64, t5942: f64, t2210: f64, t11593: f64, t1901: f64, t26868: f64, t26872: f64, t26876: f64, t26880: f64, t26885: f64, t26890: f64, t26894: f64, t26899: f64, t26902: f64, t26906: f64, t446: f64) -> (f64, f64) {
    let t26911 = t2185 * t167 * t26909;
    let t26914 = t1882 * t6710;
    let t26916 = t8392 * t6627;
    let t26918 = t5942 * t3052;
    let t26919 = t2210 * t26918;
    let t26922 = t1901 * t26868 / 9.0_f64 - t446 * t26872 / 9.0_f64 + t446 * t26876 / 3.0_f64 + t446 * t26880 / 3.0_f64 + t446 * t26885 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t26890 + 2.0_f64 / 3.0_f64 * t446 * t26894 + t446 * t26899 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t26902 + 2.0_f64 / 3.0_f64 * t446 * t26906 + 2.0_f64 / 3.0_f64 * t446 * t26911 - 2.0_f64 / 9.0_f64 * t26914 - t26916 / 27.0_f64 + 2.0_f64 / 9.0_f64 * t11593 * t26919;
    (t26918, t26922)
}
