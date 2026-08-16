//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 403/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk403(t3295: f64, t969: f64, t825: f64, t3209: f64, t836: f64, t568: f64, t3234: f64, t808: f64, t3191: f64, t325: f64, t3190: f64, t813: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3296 = t969 * t3295;
    let t3297 = t825 * t3296;
    let t3298 = 0.38342925953920749676e0_f64 * t3297;
    let t3299 = t836 * t3209;
    let t3300 = t568 * t3299;
    let t3303 = t808 * t3234;
    let t3304 = t568 * t3303;
    let t3307 = t3191 * t325;
    let t3308 = t3190 * t3307;
    let t3309 = t813 * t3308;
    (t3296, t3297, t3298, t3299, t3300, t3303, t3304, t3307, t3308, t3309)
}
