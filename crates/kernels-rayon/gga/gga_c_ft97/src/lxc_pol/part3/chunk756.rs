//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 756/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk756(t15776: f64, t35: f64, t4466: f64, t77: f64, t3020: f64, t25: f64, t4491: f64, t3066: f64, t15630: f64, t7906: f64, t1597: f64, t929: f64) -> (f64, f64, f64, f64, f64) {
    let t15777 = t15776 * t35;
    let t15781 = t77 * t4466;
    let t15782 = t3020 * t15781;
    let t15785 = t4491 * t25;
    let t15786 = t15785 * t3066;
    let t15789 = t7906 * t15630;
    let t15792 = t929 * t1597;
    (t15777, t15782, t15786, t15789, t15792)
}
