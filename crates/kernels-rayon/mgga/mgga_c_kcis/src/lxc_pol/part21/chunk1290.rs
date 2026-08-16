//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1290/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1290(t13250: f64, t4994: f64, t7718: f64, t1020: f64, t26753: f64, t4548: f64, t14447: f64, t27949: f64, t7703: f64, t27806: f64, t71743: f64, t1245: f64, t27774: f64, t2909: f64) -> (f64, f64, f64, f64, f64) {
    let t95756 = t4994 * t7718 * t13250;
    let t95759 = t1020 * t26753 * t4548;
    let t95764 = t7703 * t14447 * t27949;
    let t95769 = t27806 * t71743;
    let t95775 = t7703 * t1245 * t2909 * t27774;
    (t95756, t95759, t95764, t95769, t95775)
}
