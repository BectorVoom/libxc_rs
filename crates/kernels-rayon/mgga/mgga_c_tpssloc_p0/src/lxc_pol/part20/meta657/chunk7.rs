//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2435/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2435(t49818: f64, t10962: f64, t4630: f64, t13961: f64, t3114: f64, t10403: f64, t10863: f64, t14126: f64, t14213: f64, t14489: f64, t14491: f64, t17732: f64, t3070: f64, t3071: f64, t3109: f64, t42508: f64, t43358: f64, t4575: f64, t4636: f64, t49799: f64, t49801: f64, t49808: f64, t49810: f64, t884: f64) -> f64 {
    let t49819 = t49818 / 4608.0_f64;
    let t49820 = t10962 * t4630;
    let t49822 = t3114 * t13961;
    let t49824 = t10403 * t3071 * t17732 * t14213 / 384.0_f64 + t3070 * t3071 * t14489 * t884 / 1536.0_f64 + t49799 / 2304.0_f64 + 5.0_f64 / 3456.0_f64 * t49801 + t42508 * t14126 / 288.0_f64 + 19.0_f64 / 864.0_f64 * t43358 * t4575 - t49808 / 2304.0_f64 + t49810 / 2304.0_f64 - t10863 * t4636 / 144.0_f64 - t3109 * t14491 / 192.0_f64 - t49819 + t49820 / 1536.0_f64 + t49822 / 768.0_f64;
    t49824
}
