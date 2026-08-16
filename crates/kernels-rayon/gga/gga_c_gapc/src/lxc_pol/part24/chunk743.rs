//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 743/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk743(t19: f64, t3071: f64, t1971: f64, t2993: f64, t144: f64, t147: f64, t200: f64, t2999: f64, t5319: f64, t1338: f64, t134: f64, t647: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8837 = t3071 * t19;
    let t8838 = t1971 * t8837;
    let t8839 = t2993 * t8838;
    let t8840 = t147 * t144;
    let t8841 = t8840 * t200;
    let t8842 = t5319 * t2999;
    let t8843 = t8841 * t8842;
    let t8844 = t8839 * t8843;
    let t8846 = t134 * t1338;
    let t8847 = t647 * t8846;
    (t8837, t8838, t8841, t8843, t8844, t8847)
}
