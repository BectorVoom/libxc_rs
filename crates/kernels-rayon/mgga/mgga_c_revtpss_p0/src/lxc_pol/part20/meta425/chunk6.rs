//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1599/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1599(t43886: f64, t43888: f64, t43890: f64, t43892: f64, t43894: f64, t43896: f64, t43899: f64, t43902: f64, t43905: f64, t43947: f64, t43950: f64, t43953: f64, t43955: f64, t43957: f64) -> f64 {
    let t44082 = 0.39862222222222222223e1_f64 * t43886 - 0.12401580246913580247e1_f64 * t43888 + 0.79724444444444444446e0_f64 * t43890 + 0.15944888888888888889e1_f64 * t43892 - 0.23917333333333333333e1_f64 * t43894 - 0.39862222222222222223e0_f64 * t43896 - 0.71752000000000000002e1_f64 * t43899 + 0.71752e1_f64 * t43902 + 0.29896666666666666667e0_f64 * t43905 + 0.1151859375e0_f64 * t43947 + 0.21908444444444444444e0_f64 * t43950 + 0.295764e1_f64 * t43953 + 0.3071625e0_f64 * t43955 + 0.46074375e0_f64 * t43957;
    t44082
}
