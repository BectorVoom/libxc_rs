//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 448/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk448(t1877: f64, t268: f64, t806: f64, t1880: f64, t808: f64, t568: f64, t2166: f64, t2170: f64, t2174: f64, t2178: f64, t2182: f64, t2185: f64, t2188: f64, t2191: f64, t2194: f64, t2197: f64, t323: f64, t770: f64, t784: f64, t797: f64, t807: f64, t810: f64, t813: f64, t815: f64, t833: f64, t838: f64) -> (f64, f64, f64) {
    let t2200 = t268 * t1877;
    let t2201 = t2200 * t806;
    let t2202 = t808 * t1880;
    let t2203 = t568 * t2202;
    let t2206 = 0.47667319935800568892e0_f64 * t770 * t784 - 0.51123901271894332903e0_f64 * t323 * t2166 + 0.23005755572352449806e1_f64 * t833 * t2170 - 0.23005755572352449806e1_f64 * t813 * t2174 + 0.23005755572352449806e1_f64 * t2178 * t810 + 0.11502877786176224903e1_f64 * t807 * t2182 - 0.35750489951850426669e0_f64 * t797 * t2185 - 0.61348681526273199483e1_f64 * t813 * t2188 + 0.61348681526273199483e1_f64 * t833 * t2191 - 0.46011511144704899612e1_f64 * t2194 * t815 + 0.46011511144704899612e1_f64 * t2197 * t838 - 0.23005755572352449806e1_f64 * t2201 * t2203;
    (t2200, t2201, t2206)
}
