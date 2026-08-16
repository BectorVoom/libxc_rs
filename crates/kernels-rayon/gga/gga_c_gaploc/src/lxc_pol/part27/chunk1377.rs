//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1377/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1377(t38413: f64, t550: f64, t1358: f64, t1365: f64, t30189: f64, t30199: f64, t30207: f64, t32053: f64, t32055: f64, t32057: f64, t32059: f64, t32062: f64, t32066: f64, t32072: f64, t32074: f64, t32077: f64, t32080: f64, t32084: f64) -> (f64, f64) {
    let t38447 = t550 * t38413;
    let t38451 = 0.63233348079280332442e-2_f64 * t1358 * t1365 * t38447 + t30189 - t32053 + t32055 - t32057 + t32059 + t32062 - t30199 - t32066 - t32072 - t30207 - t32074 - t32077 - t32080 - t32084;
    (t38447, t38451)
}
