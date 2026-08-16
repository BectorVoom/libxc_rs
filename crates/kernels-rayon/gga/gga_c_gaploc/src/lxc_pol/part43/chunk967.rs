//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 967/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk967(t39145: f64, t787: f64, t32970: f64, t13870: f64, t835: f64, t723: f64, t1457: f64, t2103: f64, t13900: f64, t5771: f64, t41136: f64, t41139: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t47266 = t787 * t39145;
    let t47267 = t47266 * t32970;
    let t47270 = t835 * t13870;
    let t47271 = t47270 * t723;
    let t47274 = 0.71500979903700853338e0_f64 * t2103 * t1457 * t47271;
    let t47275 = t5771 * t13900;
    let t47280 = 0.15337170381568299871e1_f64 * t41136;
    let t47283 = 0.76685851907841499354e0_f64 * t41139;
    (t47267, t47270, t47271, t47274, t47275, t47280, t47283)
}
