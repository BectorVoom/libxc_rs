//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1083/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1083(t15001: f64, t558: f64, t15931: f64, t275: f64, t71544: f64, t71545: f64, t71546: f64, t75648: f64, t75652: f64, t75654: f64, t75658: f64, t75666: f64, t77713: f64, t77715: f64, t77717: f64, t77719: f64, t77724: f64, t77725: f64, t77726: f64, t884: f64) -> (f64, f64) {
    let t80280 = t15001 * t558;
    let t80283 = t275 * t15931;
    let t80284 = 0.93188427318671584242e-2_f64 * t75648 + 0.93188427318671584242e-2_f64 * t75652 - 0.15531404553111930707e-1_f64 * t75654 - 0.15531404553111930707e-1_f64 * t75658 + t77713 + t77715 + t77717 - t77719 + 0.59871208509319042821e-1_f64 * t884 * t80280 + t77724 + t77725 + t77726 + t75666 + t80283 - t71544 - t71545 + t71546;
    (t80280, t80284)
}
