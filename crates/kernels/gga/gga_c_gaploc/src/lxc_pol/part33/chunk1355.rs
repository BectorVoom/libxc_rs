//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1355/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1355<F: Float>(t35228: F, t10434: F, t1391: F, t2487: F, t2355: F, t8435: F, t27229: F, t7826: F, t10289: F, t10299: F, t10293: F, t10302: F) -> (F, F, F, F, F, F, F, F) {
    let t35229 = F::cast_from(0.51123901271894332902e0_f64) * t35228;
    let t35231 = t2487 * t1391 * t10434;
    let t35232 = F::cast_from(0.2698205900461089792e0_f64) * t35231;
    let t35240 = t2355 * t8435;
    let t35242 = F::cast_from(6.0_f64) * t27229 * t7826;
    let t35252 = F::cast_from(2.0_f64) * t10289;
    let t35253 = F::cast_from(4.0_f64) * t10299;
    let t35254 = F::cast_from(4.0_f64) * t10293;
    let t35255 = F::cast_from(4.0_f64) * t10302;
    (t35229, t35232, t35240, t35242, t35252, t35253, t35254, t35255)
}
