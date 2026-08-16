//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 995/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk995(t2752: f64, t31429: f64, t1914: f64, t2553: f64, t193: f64, t201: f64, t8565: f64, t2749: f64, t10143: f64, t31441: f64, t868: f64, t113114: f64, t1877: f64, t2249: f64, t22951: f64, t22960: f64, t22961: f64, t22964: f64, t23296: f64, t24191: f64, t24339: f64, t2522: f64, t25373: f64, t26756: f64, t30767: f64, t31442: f64, t31448: f64, t4314: f64, t6671: f64, t7114: f64, t81547: f64, t84791: f64, t84797: f64, t8566: f64, t8569: f64, t86716: f64, t86770: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t114992 = t31429 * t2752;
    let t115000 = t1914 * t2553;
    let t115009 = t193 * t201 * t8565;
    let t115012 = t1914 * t2749;
    let t115027 = t8565 * t10143;
    let t115030 = t31441 * t868;
    let t115040 = -t1877 * t114992 * t6671 - t1877 * t7114 * t113114 / 2.0_f64 - 3.0_f64 * t84797 * t31442 - 3.0_f64 / 2.0_f64 * t24191 * t22960 * t115000 - t1877 * t7114 * t2249 * t1914 / 2.0_f64 - 3.0_f64 * t115009 * t22961 - 3.0_f64 * t26756 * t86716 * t115012 - t1877 * t84791 * t8569 / 2.0_f64 + 2.0_f64 * t26756 * t86770 * t31448 - t1877 * t24339 * t30767 + 3.0_f64 * t2522 * t8566 * t22964 + t1877 * t115027 * t23296 + 6.0_f64 * t24191 * t25373 * t115030 + 3.0_f64 * t4314 * t8566 * t22951 - 3.0_f64 * t24191 * t81547 * t31441;
    (t114992, t115000, t115009, t115012, t115027, t115030, t115040)
}
