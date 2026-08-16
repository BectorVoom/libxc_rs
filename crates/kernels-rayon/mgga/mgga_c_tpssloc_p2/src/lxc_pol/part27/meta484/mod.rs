//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta484 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1860;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1861;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta484(t23696: f64, t23697: f64, t23661: f64, t3188: f64, t1945: f64, t3120: f64, t1060: f64, t23571: f64, t383: f64, t23384: f64, t6787: f64, t2776: f64, t6785: f64, t6784: f64, t1003: f64, t1058: f64, t1953: f64, t23346: f64, t23601: f64, t23666: f64, t23670: f64, t23674: f64, t23680: f64, t23687: f64, t23693: f64, t3076: f64, t3186: f64, t353: f64, t6680: f64, t6687: f64, t6790: f64, t6797: f64, t6802: f64, t6806: f64, t6813: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23698, t23701, t23705, t23707, t23712, t23714) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1860(t23696, t23697, t23661, t3188, t1945, t3120, t1060, t23571, t383, t23384, t6787, t2776, t6785);
        let (t23715, t23720) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1861(t23714, t6784, t1003, t1058, t1953, t23346, t23601, t23666, t23670, t23674, t23680, t23687, t23693, t23698, t23701, t23705, t23707, t23712, t3076, t3186, t353, t6680, t6687, t6787, t6790, t6797, t6802, t6806, t6813);
    (t23698, t23701, t23705, t23707, t23712, t23714, t23715, t23720)
}
