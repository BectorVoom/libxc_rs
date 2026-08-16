//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 579/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk579(t4338: f64, t4510: f64, t1409: f64, t2989: f64, t2988: f64, t2987: f64, t344: f64, t4343: f64, t3966: f64, t978: f64, t977: f64, t135: f64, t1599: f64) -> (f64, f64, f64, f64, f64) {
    let t4511 = t4510 * t4338;
    let t4514 = t2989 * t1409;
    let t4515 = t2988 * t4514;
    let t4518 = t2987 * t344;
    let t4519 = t4518 * t4343;
    let t4522 = t978 * t3966;
    let t4523 = t977 * t4522;
    let t4528 = t135 * t1599;
    (t4511, t4515, t4519, t4523, t4528)
}
