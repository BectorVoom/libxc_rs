//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 269/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk269(t215: f64, t535: f64, t782: f64, t154: f64, t547: f64, t205: f64, t1307: f64, t210: f64, t214: f64, t792: f64, t795: f64) -> (f64, f64, f64, f64) {
    let t1313 = 0.19444444444444444444e-2_f64 * t782 * t535 * t215;
    let t1314 = t154 * t547;
    let t1315 = t205 * t1314;
    let t1317 = t210 * t214 * t1307;
    let t1322 = 0.41666666666666666666e-3_f64 * t792 * t535 * t795;
    let t1323 = -t1313 - 0.16666666666666666666e-2_f64 * t1315 * t1317 - t1322;
    (t1314, t1315, t1317, t1323)
}
