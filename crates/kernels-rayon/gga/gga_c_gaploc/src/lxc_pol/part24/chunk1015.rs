//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1015/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1015(t10942: f64, t9800: f64, t10905: f64, t10908: f64, t10911: f64, t10918: f64, t10921: f64, t10923: f64, t10927: f64, t10934: f64, t10935: f64, t10937: f64, t10941: f64, t9935: f64, t9937: f64, t9942: f64, t9946: f64) -> f64 {
    let t10943 = t9800 * t10942;
    let t10944 = 0.9585731488480187419e0_f64 * t10943;
    let t10945 = t10905 - t10908 + t10911 - t10918 + t10921 + t10923 - t10927 + t10934 + t9935 + t9937 - t9942 - t9946 + t10935 + t10937 - t10941 + t10944;
    t10945
}
