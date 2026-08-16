//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1196/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1196(t31590: f64, t426: f64, t2268: f64, t535: f64, t10227: f64, t23927: f64, t10276: f64, t4141: f64, t10224: f64, t1083: f64, t30126: f64, t30129: f64, t30132: f64, t30135: f64, t30145: f64, t30148: f64, t30152: f64, t30169: f64, t30171: f64, t30173: f64, t3341: f64, t380: f64) -> f64 {
    let t32005 = t31590 * t426;
    let t32008 = 0.56910013271352299198e-1_f64 * t2268 * t535 * t32005;
    let t32009 = t23927 * t10227;
    let t32010 = 0.23712505529730124666e-2_f64 * t32009;
    let t32012 = 0.9485002211892049866e-2_f64 * t4141 * t10276;
    let t32017 = t30126 + t30129 - t30132 + t32008 + t32010 + t30135 - t30145 + t30148 - t30152 - t32012 - t30169 + 0.7588001769513639893e-1_f64 * t1083 * t3341 + 0.7588001769513639893e-1_f64 * t380 * t10224 + t30171 - t30173;
    t32017
}
