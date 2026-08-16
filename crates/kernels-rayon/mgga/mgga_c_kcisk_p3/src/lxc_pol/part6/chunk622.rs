//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 622/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk622(t1676: f64, t1685: f64, t8607: f64, t4787: f64, t8590: f64, t4790: f64, t1674: f64, t2396: f64, t45: f64, t6851: f64, t8546: f64, t8548: f64, t8552: f64, t8576: f64, t8579: f64, t8585: f64, t8592: f64) -> (f64, f64, f64) {
    let t8609 = t1676 * t8607 * t1685;
    let t8612 = t4787 * t8590;
    let t8613 = t8612 * t4790;
    let t8616 = -t8546 + t8548 - t8552 + t8576 + t8579 + 0.19751789702565206229e-1_f64 * t45 * t8585 - 0.11696446794910408142e1_f64 * t6851 * t2396 + 0.11696446794910408142e1_f64 * t1674 * t8592 - 0.58482233974552040708e0_f64 * t1674 * t8609 - 0.17315755899375863299e2_f64 * t1674 * t8613;
    (t8609, t8613, t8616)
}
