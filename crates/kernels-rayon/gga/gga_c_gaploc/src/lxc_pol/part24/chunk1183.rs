//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1183/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1183(t1367: f64, t31543: f64, t196: f64, t21488: f64, t555: f64, t2787: f64, t6509: f64, t590: f64, t1570: f64, t10177: f64, t4538: f64, t189: f64, t3394: f64, t488: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31764 = t31543 * t1367;
    let t31766 = 0.44263343655496232709e-1_f64 * t21488 * t196 * t555 * t31764;
    let t31769 = t2787 * t6509;
    let t31770 = t590 * t31769;
    let t31772 = 0.7588001769513639893e-1_f64 * t21488 * t196 * t1570 * t31770;
    let t31775 = t590 * t10177;
    let t31777 = 0.37940008847568199465e-1_f64 * t21488 * t196 * t4538 * t31775;
    let t31783 = 0.63233348079280332442e-2_f64 * t21488 * t196 * t189 * t6509 * t3394 * t488;
    (t31764, t31766, t31770, t31772, t31775, t31777, t31783)
}
