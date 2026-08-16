//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1117/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1117(t11854: f64, t7553: f64, t1078: f64, t2387: f64, t3756: f64, t33801: f64, t33803: f64, t33808: f64, t33810: f64, t33812: f64, t33815: f64, t33818: f64, t33820: f64, t33823: f64) -> f64 {
    let t33825 = t7553 * t11854;
    let t33828 = t2387 * t3756 * t1078;
    let t33830 = 0.20047434126173032506e-6_f64 * t33801 - 0.10551281119038438161e-7_f64 * t33803 - 0.11049275749843950005e-7_f64 * t33808 - 0.2750785565527147423e-6_f64 * t33810 + 0.20240885416666666668e-4_f64 * t33812 + 0.28960308421505737848e-5_f64 * t33815 - 0.25340269868817520617e-3_f64 * t33818 - 0.17376185052903442709e-3_f64 * t33820 + 0.28960308421505737848e-5_f64 * t33823 - 0.25340269868817520617e-3_f64 * t33825 - 0.17376185052903442709e-3_f64 * t33828;
    t33830
}
