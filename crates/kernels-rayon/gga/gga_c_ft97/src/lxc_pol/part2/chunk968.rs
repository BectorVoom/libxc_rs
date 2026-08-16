//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 968/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk968(t1775: f64, t4220: f64, t2: f64, t4129: f64, t2681: f64, t824: f64, t2347: f64, t852: f64, t3886: f64, t2360: f64, t1212: f64, t2781: f64) -> (f64, f64, f64, f64, f64) {
    let t15028 = 4.0_f64 / 3.0_f64 * t1775 * t4220;
    let t15037 = t2 * t4129;
    let t15039 = t2681 * t15037 * t824;
    let t15042 = t852 * t2347;
    let t15043 = t3886 * t824;
    let t15044 = t15042 * t15043;
    let t15047 = t852 * t2360;
    let t15048 = t15047 * t15043;
    let t15051 = t2781 * t1212;
    (t15028, t15039, t15044, t15048, t15051)
}
