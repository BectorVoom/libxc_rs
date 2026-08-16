//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1083/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1083(t26791: f64, t2911: f64, t5329: f64, t1086: f64, t1094: f64, t1122: f64, t303: f64, t26760: f64, t3205: f64, t1020: f64, t3213: f64, t7718: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26792 = t26791 * t2911;
    let t26793 = t5329 * t26792;
    let t26796 = t1086 * t1094;
    let t26797 = t26796 * t1122;
    let t26798 = t303 * t26797;
    let t26800 = t26760 * t3205;
    let t26801 = t1020 * t26800;
    let t26803 = t7718 * t3213;
    (t26792, t26793, t26796, t26797, t26798, t26800, t26801, t26803)
}
