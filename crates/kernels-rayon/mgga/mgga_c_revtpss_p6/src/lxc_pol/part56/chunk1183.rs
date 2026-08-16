//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1183/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1183(t1294: f64, t1769: f64, t105270: f64, t1238: f64, t124612: f64, t124626: f64, t124645: f64, t124711: f64, t124719: f64, t124748: f64, t124755: f64, t124862: f64, t131608: f64, t131611: f64, t131616: f64, t131620: f64, t131629: f64, t32015: f64, t33477: f64, t33478: f64, t33512: f64, t5236: f64, t5304: f64, t5422: f64, t7627: f64, t7652: f64, t8208: f64) -> (f64, f64) {
    let t131631 = t1769 * t1294;
    let t131640 = -0.3427184259906141157e1_f64 * t33477 * t33478 * t8208 * t7627 - 0.34694512752820797848e1_f64 * t124626 * t7652 * t5236 - 0.12395776403017003607e-3_f64 * t124719 - 0.20659627338361672678e-3_f64 * t131608 * t5304 - 0.29749863367240808656e-2_f64 * t131611 * t1238 + 0.24791552806034007213e-3_f64 * t131616 - 0.12548651892657985333e-3_f64 * t124748 - 0.19833242244827205771e-2_f64 * t131620 * t33512 + 0.56468933516960933998e-3_f64 * t124755 * t32015 * t124612 * t5422 - 0.3718732920905101082e-3_f64 * t131629 - 0.56468933516960933998e-3_f64 * t124711 * t32015 * t124612 * t131631 - 0.112937867033921868e-2_f64 * t124862 * t32015 * t124645 * t105270;
    (t131631, t131640)
}
