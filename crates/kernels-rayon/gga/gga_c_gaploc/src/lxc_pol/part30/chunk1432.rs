//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1432/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1432(t224: f64, t32721: f64, t32741: f64, t33983: f64, t35243: f64, t10289: f64, t10299: f64, t10293: f64, t10302: f64, t10625: f64, t10292: f64, t11143: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t35246 = t224 * (t32721 + t32741 + t33983 + t35243);
    let t35252 = 2.0_f64 * t10289;
    let t35253 = 4.0_f64 * t10299;
    let t35254 = 4.0_f64 * t10293;
    let t35255 = 4.0_f64 * t10302;
    let t35256 = 2.0_f64 * t10625;
    let t35257 = 2.0_f64 * t10292;
    let t35259 = 2.0_f64 * t11143;
    (t35246, t35252, t35253, t35254, t35255, t35256, t35257, t35259)
}
