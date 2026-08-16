//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 655/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk655(t273: f64, t57: f64, t1509: f64, t424: f64, t41: f64, t1477: f64, t1485: f64, t1483: f64, t400: f64, t1384: f64, t1409: f64, t452: f64) -> (f64, f64, f64, f64, f64) {
    let t4145 = t57 * t273;
    let t4694 = t424 * t1509;
    let t4695 = t41 * t4694;
    let t4700 = t1477 * t1485;
    let t4702 = t1483 * t4700 * t400;
    let t4703 = 0.48245938496077605201e2_f64 * t4702;
    let t4704 = t1409 * t1384;
    let t4705 = t4704 * t452;
    (t4145, t4695, t4703, t4704, t4705)
}
