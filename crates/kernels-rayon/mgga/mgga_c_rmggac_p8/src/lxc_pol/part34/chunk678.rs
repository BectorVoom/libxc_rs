//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 678/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk678(t14132: f64, t68541: f64, t14251: f64, t68524: f64, t14162: f64, t7254: f64, t1986: f64, t2092: f64, t24983: f64, t3129: f64, t14046: f64, t14367: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t68542 = t68541 * t14132;
    let t68543 = 0.16351352353374609375e-5_f64 * t68542;
    let t68549 = t68524 * t14251;
    let t68550 = 0.11634323970834742769e-3_f64 * t68549;
    let t68552 = t7254 * t14162;
    let t68555 = t1986 * t2092;
    let t68575 = 1.0_f64 / t3129 / t24983;
    let t68581 = t14046 * t14367;
    (t68543, t68550, t68552, t68555, t68575, t68581)
}
