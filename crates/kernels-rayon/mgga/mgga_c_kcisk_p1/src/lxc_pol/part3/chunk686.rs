//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 686/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk686(t4786: f64, t596: f64, t10552: f64, t4790: f64, t1675: f64, t4789: f64, t599: f64, t1644: f64, t4696: f64, t1665: f64, t4699: f64, t4737: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10690 = 1.0_f64 / t4786 / t596;
    let t10692 = t10690 * t10552 * t4790;
    let t10696 = 1.0_f64 / t4786 / t1675;
    let t10699 = 1.0_f64 / t4789 / t599;
    let t10700 = t10696 * t10552 * t10699;
    let t10705 = t4696 * t1644;
    let t10707 = 3.0_f64 * t10705 * t1665;
    let t10709 = 3.0_f64 * t4699 * t4737;
    (t10690, t10692, t10696, t10699, t10700, t10707, t10709)
}
