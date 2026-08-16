//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1653/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1653(t43886: f64, t43888: f64, t43890: f64, t43892: f64, t43894: f64, t43896: f64, t43899: f64, t43902: f64, t43905: f64, t43947: f64, t43950: f64, t43953: f64, t43955: f64, t43957: f64) -> f64 {
    let t45149 = 0.68863333333333333334e1_f64 * t43886 - 0.21424148148148148148e1_f64 * t43888 + 0.13772666666666666666e1_f64 * t43890 + 0.27545333333333333333e1_f64 * t43892 - 0.41318e1_f64 * t43894 - 0.68863333333333333332e0_f64 * t43896 - 0.123954e2_f64 * t43899 + 0.123954e2_f64 * t43902 + 0.516475e0_f64 * t43905 + 0.2366859375e0_f64 * t43947 + 0.27785333333333333334e0_f64 * t43950 + 0.375102e1_f64 * t43953 + 0.6311625e0_f64 * t43955 + 0.94674375e0_f64 * t43957;
    t45149
}
