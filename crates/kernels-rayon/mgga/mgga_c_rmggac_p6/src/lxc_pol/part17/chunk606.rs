//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 606/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk606(t2084: f64, t570: f64, t27: f64, t2145: f64, t551: f64, t2134: f64, t2060: f64, t8377: f64, t1550: f64, t1632: f64, t645: f64, t3928: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8532 = t2084 * t570;
    let t8533 = t27 * t8532;
    let t8534 = t2145 * t8533;
    let t8536 = t2084 * t551;
    let t8537 = t27 * t8536;
    let t8538 = t2134 * t8537;
    let t8542 = t2060 * t8377;
    let t8543 = t1550 * t8542;
    let t8545 = t645 * t1632;
    let t8546 = t3928 * t8545;
    (t8533, t8534, t8537, t8538, t8542, t8543, t8545, t8546)
}
