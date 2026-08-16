//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 905/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk905(t6188: f64, t772: f64, t3243: f64, t10247: f64, t10250: f64, t10253: f64, t10258: f64, t10262: f64, t10267: f64, t10271: f64, t10274: f64, t10276: f64, t10278: f64) -> (f64, f64) {
    let t10280 = t772 * t6188;
    let t10281 = t3243 * t10280;
    let t10283 = 0.69596735221749395468e-7_f64 * t10247 - 0.2087902056652481864e-5_f64 * t10250 - 0.11742981196020707897e-5_f64 * t10253 - 0.74922666485027954031e-6_f64 * t10258 - 0.12374299522427042515e-6_f64 * t10262 + 0.2087902056652481864e-5_f64 * t10267 - 0.11742981196020707897e-4_f64 * t10271 - 0.33406432906439709826e-4_f64 * t10274 + 0.74372214241464483348e-4_f64 * t10276 + 0.23404413911513494211e-4_f64 * t10278 - 0.11742981196020707897e-5_f64 * t10281;
    (t10281, t10283)
}
