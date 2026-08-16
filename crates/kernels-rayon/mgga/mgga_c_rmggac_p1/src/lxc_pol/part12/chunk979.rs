//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 979/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk979(t40731: f64, t7720: f64, t321: f64, t8924: f64, t262: f64, t7204: f64, t333: f64, t7192: f64, t1970: f64, t236: f64, t498: f64, t5605: f64, t7231: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t40732 = t7720 * t40731;
    let t40734 = t8924 * t321;
    let t40735 = t262 * t40734;
    let t40736 = t7204 * t40735;
    let t40738 = t8924 * t333;
    let t40739 = t262 * t40738;
    let t40740 = t7192 * t40739;
    let t40747 = t1970 * t7231 * t236 * t5605 * t498;
    (t40732, t40734, t40735, t40736, t40738, t40739, t40740, t40747)
}
