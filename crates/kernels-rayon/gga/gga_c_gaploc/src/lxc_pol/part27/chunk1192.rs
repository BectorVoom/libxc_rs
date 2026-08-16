//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1192/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1192(t32044: f64, t10262: f64, t2312: f64, t23983: f64, t2761: f64, t6455: f64, t10167: f64, t29874: f64, t10269: f64, t4141: f64, t10196: f64, t3833: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32045 = 0.11856252764865062333e-2_f64 * t32044;
    let t32046 = t2312 * t10262;
    let t32047 = 0.23712505529730124666e-2_f64 * t32046;
    let t32049 = t23983 * t2761 * t6455;
    let t32050 = 0.23712505529730124666e-2_f64 * t32049;
    let t32052 = t29874 * t10167;
    let t32053 = 0.71137516589190373998e-2_f64 * t32052;
    let t32055 = 0.63233348079280332441e-2_f64 * t4141 * t10269;
    let t32057 = 0.56910013271352299198e-1_f64 * t3833 * t10196;
    (t32045, t32047, t32050, t32053, t32055, t32057)
}
