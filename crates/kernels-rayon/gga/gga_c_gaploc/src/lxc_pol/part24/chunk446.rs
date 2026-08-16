//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 446/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk446(t1512: f64, t325: f64, t61: f64, t1710: f64, t836: f64, t568: f64, t808: f64, t679: f64, t685: f64, t806: f64, t1835: f64, t1716: f64, t531: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2166 = t61 * t1512 * t325;
    let t2169 = t836 * t1710;
    let t2170 = t568 * t2169;
    let t2173 = t808 * t1710;
    let t2174 = t568 * t2173;
    let t2177 = t679 * t685;
    let t2178 = t2177 * t806;
    let t2181 = t808 * t1835;
    let t2182 = t568 * t2181;
    let t2185 = t531 * t1716;
    (t2166, t2170, t2174, t2177, t2178, t2182, t2185)
}
