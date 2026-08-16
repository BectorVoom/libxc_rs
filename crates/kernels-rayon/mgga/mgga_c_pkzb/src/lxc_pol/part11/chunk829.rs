//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 829/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk829(t7033: f64, t7038: f64, t7040: f64, t5179: f64, t5187: f64, t4996: f64, t5005: f64, t5011: f64, t5019: f64, t5022: f64, t5154: f64, t5186: f64, t7030: f64, t7037: f64, t7042: f64, t8795: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8842 = 0.34631718211362927517e2_f64 * t7033;
    let t8843 = 0.11696447245269292414e1_f64 * t7038;
    let t8844 = 0.23392894490538584828e1_f64 * t7040;
    let t8845 = 12.0_f64 * t5179;
    let t8846 = 32.0_f64 * t5187;
    let t8847 = t7030 - t5154 - t8795 + t4996 + t5005 - t5011 - t8842 - t7037 - t8843 + t8844 + t5019 - t5022 - t7042 + t8845 + t5186 + t8846;
    (t8842, t8843, t8844, t8845, t8846, t8847)
}
