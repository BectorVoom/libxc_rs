//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 602/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk602(t2441: f64, t4917: f64, t420: f64, t701: f64, t2446: f64, t4635: f64, t704: f64, t2435: f64, t3796: f64, t3804: f64, t5031: f64, t5034: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5037 = t2441 * t4917;
    let t5038 = t420 * t5037;
    let t5039 = t701 * t5038;
    let t5041 = t2446 * t4917;
    let t5042 = t420 * t5041;
    let t5043 = t701 * t5042;
    let t5045 = t704 * t4635;
    let t5046 = t420 * t5045;
    let t5047 = t701 * t5046;
    let t5049 = 0.18727458458024691358e0_f64 * t5031 - 0.3404992446913580247e-1_f64 * t3796 - 0.3404992446913580247e-1_f64 * t5034 - t2435 + 0.42562405586419753086e-2_f64 * t3804 + 0.85124811172839506173e-2_f64 * t5039 - 0.12768721675925925926e-1_f64 * t5043 + 0.6384360837962962963e-2_f64 * t5047;
    (t5037, t5039, t5041, t5043, t5045, t5047, t5049)
}
