//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta598 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2059;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta598(t7642: f64, t96873: f64, t26948: f64, t487: f64, t8945: f64, t26936: f64, t3736: f64, t7635: f64, t3566: f64, t1203: f64, t1294: f64, t1209: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t97034, t97041, t97050, t97065, t97066, t97067, t97078) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2059(t7642, t96873, t26948, t487, t8945, t26936, t3736, t7635, t3566, t1203, t1294, t1209);
    (t97034, t97041, t97050, t97065, t97066, t97067, t97078)
}
