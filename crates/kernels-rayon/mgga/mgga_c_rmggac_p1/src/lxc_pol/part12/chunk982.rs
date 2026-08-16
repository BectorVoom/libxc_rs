//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 982/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk982(t1652: f64, t7567: f64, t352: f64, t8915: f64, t5148: f64, t333: f64, t4669: f64, t2392: f64, t876: f64, t27048: f64, t551: f64, t7858: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t40791 = t7567 * t1652;
    let t40802 = t8915 * t352;
    let t40803 = t5148 * t40802;
    let t40804 = 0.15965655602485078085e0_f64 * t40803;
    let t40805 = t8915 * t333;
    let t40806 = t4669 * t40805;
    let t40807 = 0.23948483403727617128e0_f64 * t40806;
    let t40808 = t2392 * t876;
    let t40809 = t27048 * t40808;
    let t40811 = t7858 * t551;
    (t40791, t40802, t40804, t40805, t40807, t40808, t40809, t40811)
}
