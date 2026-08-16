//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1200/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1200(t32044: f64, t10262: f64, t2312: f64, t23983: f64, t2761: f64, t6455: f64, t10172: f64, t30182: f64, t30184: f64, t30186: f64, t32021: f64, t32025: f64, t32028: f64, t32036: f64, t32038: f64, t32041: f64, t32043: f64, t4141: f64) -> f64 {
    let t32045 = 0.11856252764865062333e-2_f64 * t32044;
    let t32046 = t2312 * t10262;
    let t32047 = 0.23712505529730124666e-2_f64 * t32046;
    let t32049 = t23983 * t2761 * t6455;
    let t32050 = 0.23712505529730124666e-2_f64 * t32049;
    let t32051 = -t30182 + t32021 - t32025 + t32028 + 0.31616674039640166222e-2_f64 * t4141 * t10172 + t32036 + t32038 - t32041 + t32043 + t32045 + t32047 - t30184 + t30186 + t32050;
    t32051
}
