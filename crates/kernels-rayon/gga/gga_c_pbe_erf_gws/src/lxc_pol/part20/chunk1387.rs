//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1387/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1387(t3931: f64, t810: f64, t3703: f64, t944: f64, t15392: f64, t321: f64, t1105: f64, t1167: f64, t11737: f64, t13751: f64, t13756: f64, t14153: f64, t14390: f64, t14821: f64, t2494: f64, t3324: f64, t3944: f64, t3946: f64, t4062: f64, t4063: f64, t52799: f64, t52861: f64, t52884: f64, t52887: f64, t56059: f64, t9807: f64) -> f64 {
    let t57820 = t3931 * t810;
    let t57830 = t3703 * t944;
    let t57840 = t321 * t15392;
    let t57853 = 6.0_f64 * t1105 * t3946 * t52799 - 2.0_f64 * t1167 * t4062 * t52861 + 6.0_f64 * t11737 * t13756 * t3944 + 6.0_f64 * t13751 * t13756 * t3703 - 6.0_f64 * t13756 * t4063 * t57830 + 6.0_f64 * t14153 * t3946 * t57820 + 6.0_f64 * t14390 * t2494 * t3946 - 2.0_f64 * t14821 * t3324 * t4062 + 3.0_f64 * t3944 * t3946 * t9807 + 3.0_f64 * t3946 * t56059 * t810 + t52884 + t52887 - t57840;
    t57853
}
