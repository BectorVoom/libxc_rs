//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 990/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk990(t18102: f64, t894: f64, t18019: f64, t3245: f64, t18030: f64, t3235: f64, t15786: f64, t17921: f64, t15274: f64, t18023: f64, t3087: f64, t914: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t18103 = t894 * t18102;
    let t18106 = t3245 * t18019;
    let t18114 = t3235 * t18030;
    let t18117 = t15786 * t17921;
    let t18120 = t15786 * t15274;
    let t18130 = t3087 * t18023;
    let t18131 = t914 * t18130;
    (t18103, t18106, t18114, t18117, t18120, t18130, t18131)
}
