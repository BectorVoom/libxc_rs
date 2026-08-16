//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2606/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2606(t11791: f64, t5024: f64, t11820: f64, t5002: f64, t11153: f64, t4899: f64, t3540: f64, t4961: f64, t11709: f64, t15640: f64, t1227: f64, t13969: f64, t15611: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t52991 = t5024 * t11791;
    let t52993 = t5002 * t11820;
    let t52995 = t4899 * t11153;
    let t52999 = t4961 * t3540;
    let t53001 = t11709 * t15640;
    let t53023 = t1227 * t13969 * t15611;
    (t52991, t52993, t52995, t52999, t53001, t53023)
}
