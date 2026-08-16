//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 985/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk985(t1454: f64, t6837: f64, t1403: f64, t35266: f64, t681: f64, t35269: f64, t107910: f64, t1091: f64, t109713: f64, t140513: f64, t141422: f64, t193: f64, t2354: f64, t24204: f64, t24231: f64, t27993: f64, t28001: f64, t28026: f64, t28033: f64, t28039: f64, t33499: f64, t35251: f64, t6002: f64, t6008: f64, t684: f64) -> f64 {
    let t149715 = t6837 * t1454;
    let t149725 = t1403 * t681 * t35266;
    let t149728 = t1403 * t681 * t35269;
    let t149738 = -t6002 * t140513 * t28026 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t6002 * t24231 * t28001 - t33499 * t27993 / 18.0_f64 - t24204 * t35251 / 18.0_f64 - t6002 * t2354 * t141422 * t1091 / 18.0_f64 - t6002 * t2354 * t149715 * t684 / 9.0_f64 + t33499 * t28033 / 9.0_f64 - t33499 * t28039 / 27.0_f64 - t149725 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t149728 - 2.0_f64 / 3.0_f64 * t1403 * t193 * t6008 * t109713 - 2.0_f64 / 3.0_f64 * t1403 * t193 * t6008 * t107910;
    t149738
}
