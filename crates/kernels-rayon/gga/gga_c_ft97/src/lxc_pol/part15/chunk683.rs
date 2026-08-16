//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 683/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk683(t20080: f64, t419: f64, t20044: f64, t423: f64, t420: f64, t11299: f64, t15840: f64, t15855: f64, t15866: f64, t20067: f64, t20071: f64, t20074: f64, t20078: f64, t8079: f64) -> (f64, f64, f64, f64) {
    let t20081 = t419 * t20080;
    let t20083 = t423 * t20044;
    let t20084 = t420 * t20083;
    let t20085 = t419 * t20084;
    let t20087 = t8079 - 0.42562405586419753086e-2_f64 * t11299 + 0.85124811172839506172e-2_f64 * t15840 - 0.12768721675925925926e-1_f64 * t15855 + 0.63843608379629629629e-2_f64 * t15866 + 0.19862455940329218107e-1_f64 * t20067 - 0.51074886703703703704e-1_f64 * t20071 + 0.25537443351851851852e-1_f64 * t20074 + 0.38306165027777777778e-1_f64 * t20078 - 0.38306165027777777778e-1_f64 * t20081 + 0.6384360837962962963e-2_f64 * t20085;
    (t20081, t20083, t20085, t20087)
}
