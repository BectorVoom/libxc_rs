//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 967/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk967<F: Float>(t39145: F, t787: F, t32970: F, t13870: F, t835: F, t723: F, t1457: F, t2103: F, t13900: F, t5771: F, t41136: F, t41139: F) -> (F, F, F, F, F, F, F) {
    let t47266 = t787 * t39145;
    let t47267 = t47266 * t32970;
    let t47270 = t835 * t13870;
    let t47271 = t47270 * t723;
    let t47274 = F::cast_from(0.71500979903700853338e0_f64) * t2103 * t1457 * t47271;
    let t47275 = t5771 * t13900;
    let t47280 = F::cast_from(0.15337170381568299871e1_f64) * t41136;
    let t47283 = F::cast_from(0.76685851907841499354e0_f64) * t41139;
    (t47267, t47270, t47271, t47274, t47275, t47280, t47283)
}
