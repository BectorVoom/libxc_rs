//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1090/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1090(t25462: f64, t35814: f64, t10248: f64, t10683: f64, t1091: f64, t142485: f64, t142663: f64, t142913: f64, t25459: f64, t2665: f64, t28501: f64, t29000: f64, t33996: f64, t34001: f64, t34006: f64, t36109: f64, t3746: f64, t4162: f64, t44280: f64, t6216: f64, t6217: f64, t6967: f64) -> f64 {
    let t152590 = t25462 * t35814;
    let t152615 = -4.0_f64 * t6216 * t44280 * t34006 * t4162 + 2.0_f64 * t6216 * t10683 * t6217 * t28501 - t25459 * t36109 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t29000 * t2665 * t33996 * t3746 - t152590 / 27.0_f64 + t6216 * t10683 * t34001 * t4162 + t6216 * t10248 * t142913 * t1091 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t29000 * t10248 * t34006 * t3746 - t142663 * t6967 / 18.0_f64 - t6216 * t2665 * t142485 * t1091 / 18.0_f64 + t29000 * t2665 * t34001 * t3746 / 9.0_f64 + t25459 * t35814 / 9.0_f64;
    t152615
}
