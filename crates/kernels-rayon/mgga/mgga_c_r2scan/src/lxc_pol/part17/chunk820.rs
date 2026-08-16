//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 820/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk820(t4987: f64, t8590: f64, t88: f64, t41: f64, t3034: f64, t457: f64, t4791: f64, t4794: f64, t4798: f64, t4806: f64, t4972: f64, t4975: f64, t4979: f64, t4981: f64, t4992: f64, t6961: f64, t8559: f64, t8560: f64, t8592: f64) -> (f64, f64, f64, f64) {
    let t8634 = 0.17315859105681463759e2_f64 * t4987;
    let t8635 = t8590 * t88;
    let t8636 = t41 * t8635;
    let t8637 = t3034 * t457;
    let t8638 = t41 * t8637;
    let t8639 = t4972 - t4975 + t8559 + t8560 - t4979 + t4981 + t6961 + t8592 + t4791 - t4794 - t4798 + t4806 - t8634 - t4992 + t8636 + t8638;
    (t8634, t8636, t8638, t8639)
}
