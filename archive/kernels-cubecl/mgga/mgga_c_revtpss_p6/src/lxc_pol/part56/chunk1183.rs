//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1183/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1183<F: Float>(t1294: F, t1769: F, t105270: F, t1238: F, t124612: F, t124626: F, t124645: F, t124711: F, t124719: F, t124748: F, t124755: F, t124862: F, t131608: F, t131611: F, t131616: F, t131620: F, t131629: F, t32015: F, t33477: F, t33478: F, t33512: F, t5236: F, t5304: F, t5422: F, t7627: F, t7652: F, t8208: F) -> (F, F) {
    let t131631 = t1769 * t1294;
    let t131640 = -F::cast_from(0.3427184259906141157e1_f64) * t33477 * t33478 * t8208 * t7627 - F::cast_from(0.34694512752820797848e1_f64) * t124626 * t7652 * t5236 - F::cast_from(0.12395776403017003607e-3_f64) * t124719 - F::cast_from(0.20659627338361672678e-3_f64) * t131608 * t5304 - F::cast_from(0.29749863367240808656e-2_f64) * t131611 * t1238 + F::cast_from(0.24791552806034007213e-3_f64) * t131616 - F::cast_from(0.12548651892657985333e-3_f64) * t124748 - F::cast_from(0.19833242244827205771e-2_f64) * t131620 * t33512 + F::cast_from(0.56468933516960933998e-3_f64) * t124755 * t32015 * t124612 * t5422 - F::cast_from(0.3718732920905101082e-3_f64) * t131629 - F::cast_from(0.56468933516960933998e-3_f64) * t124711 * t32015 * t124612 * t131631 - F::cast_from(0.112937867033921868e-2_f64) * t124862 * t32015 * t124645 * t105270;
    (t131631, t131640)
}
