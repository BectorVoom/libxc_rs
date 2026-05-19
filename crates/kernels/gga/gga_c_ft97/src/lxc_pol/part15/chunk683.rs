//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 683/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk683<F: Float>(t20080: F, t419: F, t20044: F, t423: F, t420: F, t11299: F, t15840: F, t15855: F, t15866: F, t20067: F, t20071: F, t20074: F, t20078: F, t8079: F) -> (F, F, F, F) {
    let t20081 = t419 * t20080;
    let t20083 = t423 * t20044;
    let t20084 = t420 * t20083;
    let t20085 = t419 * t20084;
    let t20087 = t8079 - F::cast_from(0.42562405586419753086e-2_f64) * t11299 + F::cast_from(0.85124811172839506172e-2_f64) * t15840 - F::cast_from(0.12768721675925925926e-1_f64) * t15855 + F::cast_from(0.63843608379629629629e-2_f64) * t15866 + F::cast_from(0.19862455940329218107e-1_f64) * t20067 - F::cast_from(0.51074886703703703704e-1_f64) * t20071 + F::cast_from(0.25537443351851851852e-1_f64) * t20074 + F::cast_from(0.38306165027777777778e-1_f64) * t20078 - F::cast_from(0.38306165027777777778e-1_f64) * t20081 + F::cast_from(0.6384360837962962963e-2_f64) * t20085;
    (t20081, t20083, t20085, t20087)
}
