//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1238/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1238(t17449: f64, t491: f64, t11825: f64, t27543: f64, t1928: f64, t4248: f64, t5747: f64, t5998: f64, t1528: f64, t4254: f64, t572: f64, t2060: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t97701 = t17449 * t491;
    let t97706 = t11825 * t27543;
    let t97727 = t4248 * t1928;
    let t97767 = t5747 * t27543;
    let t97772 = t5998 * t491;
    let t97784 = t1528 * t1928;
    let t97793 = t4254 * t572;
    let t97800 = sigma2 * t2060;
    (t97701, t97706, t97727, t97767, t97772, t97784, t97793, t97800)
}
