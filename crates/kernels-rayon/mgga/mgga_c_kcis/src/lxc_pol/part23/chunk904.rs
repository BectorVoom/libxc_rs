//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 904/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk904(t498: f64, t5713: f64, t16078: f64, t16069: f64, t5701: f64, t12125: f64, t12129: f64, t12131: f64, t12145: f64, t12149: f64, t1368: f64, t16874: f64, t16878: f64, t16881: f64, t16886: f64, t16889: f64) -> (f64, f64) {
    let t16892 = t5713 * t498;
    let t16893 = t16892 * t16078;
    let t16896 = t5701 * t16069;
    let t16899 = -t12125 / 288.0_f64 + t12131 / 216.0_f64 + t12145 / 144.0_f64 + t12129 - t12149 / 432.0_f64 - t1368 * t16874 / 36.0_f64 + t1368 * t16878 / 144.0_f64 + t1368 * t16881 / 48.0_f64 + t1368 * t16886 / 72.0_f64 - t1368 * t16889 / 144.0_f64 + t1368 * t16893 / 36.0_f64 + t1368 * t16896 / 216.0_f64;
    (t16892, t16899)
}
