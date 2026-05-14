//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1191/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1191<F: Float>(t35215: F, t544: F, t32745: F, t488: F, t4391: F, t549: F, t7893: F, t10430: F, t2487: F, t6985: F, t10434: F, t1391: F, t2355: F, t8435: F, t27229: F, t7826: F) -> (F, F, F, F, F, F) {
    let t35216 = t544 * t35215;
    let t35219 = 0.79445533226334281486e-1 * t35216 * t32745 * t488;
    let t35225 = t4391 * t549 * t7893;
    let t35226 = 0.11916829983950142223e0 * t35225;
    let t35228 = t2487 * t6985 * t10430;
    let t35229 = 0.51123901271894332902e0 * t35228;
    let t35231 = t2487 * t1391 * t10434;
    let t35232 = 0.2698205900461089792e0 * t35231;
    let t35240 = t2355 * t8435;
    let t35242 = 6.0 * t27229 * t7826;
    (t35219, t35226, t35229, t35232, t35240, t35242)
}
