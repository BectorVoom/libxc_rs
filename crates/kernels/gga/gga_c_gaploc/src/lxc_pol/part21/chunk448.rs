//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 448/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk448<F: Float>(t1877: F, t268: F, t806: F, t1880: F, t808: F, t568: F, t2166: F, t2170: F, t2174: F, t2178: F, t2182: F, t2185: F, t2188: F, t2191: F, t2194: F, t2197: F, t323: F, t770: F, t784: F, t797: F, t807: F, t810: F, t813: F, t815: F, t833: F, t838: F) -> (F, F, F) {
    let t2200 = t268 * t1877;
    let t2201 = t2200 * t806;
    let t2202 = t808 * t1880;
    let t2203 = t568 * t2202;
    let t2206 = F::new(0.47667319935800568892e0) * t770 * t784 - F::new(0.51123901271894332903e0) * t323 * t2166 + F::new(0.23005755572352449806e1) * t833 * t2170 - F::new(0.23005755572352449806e1) * t813 * t2174 + F::new(0.23005755572352449806e1) * t2178 * t810 + F::new(0.11502877786176224903e1) * t807 * t2182 - F::new(0.35750489951850426669e0) * t797 * t2185 - F::new(0.61348681526273199483e1) * t813 * t2188 + F::new(0.61348681526273199483e1) * t833 * t2191 - F::new(0.46011511144704899612e1) * t2194 * t815 + F::new(0.46011511144704899612e1) * t2197 * t838 - F::new(0.23005755572352449806e1) * t2201 * t2203;
    (t2200, t2201, t2206)
}
