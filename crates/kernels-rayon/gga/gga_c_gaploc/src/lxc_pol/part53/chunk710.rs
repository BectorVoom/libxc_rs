//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 710/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk710(t13728: f64, t2343: f64, t2268: f64, t11977: f64, t888: f64, t3691: f64, t894: f64, t11986: f64, t2325: f64, t883: f64, t882: f64, t12404: f64, t12405: f64, t12783: f64, t12784: f64, t12787: f64, t12788: f64, t12789: f64, t12790: f64, t12791: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13729 = t2343 * t13728;
    let t13730 = t2268 * t13729;
    let t13732 = t11977 * t888;
    let t13733 = t2268 * t13732;
    let t13735 = t894 * t3691;
    let t13736 = t2268 * t13735;
    let t13740 = t2325 * t883 * t11986;
    let t13741 = t882 * t13740;
    let t13749 = t12783 + t12784 / 2.0_f64 + t12404 - t12405 - t12787 - t12788 + t12789 + t12790 + t12791;
    (t13729, t13730, t13732, t13733, t13735, t13736, t13740, t13741, t13749)
}
