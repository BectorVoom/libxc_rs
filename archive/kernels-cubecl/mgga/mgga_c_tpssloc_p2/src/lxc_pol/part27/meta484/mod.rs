//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta484 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1860;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1861;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta484<F: Float>(t23696: F, t23697: F, t23661: F, t3188: F, t1945: F, t3120: F, t1060: F, t23571: F, t383: F, t23384: F, t6787: F, t2776: F, t6785: F, t6784: F, t1003: F, t1058: F, t1953: F, t23346: F, t23601: F, t23666: F, t23670: F, t23674: F, t23680: F, t23687: F, t23693: F, t3076: F, t3186: F, t353: F, t6680: F, t6687: F, t6790: F, t6797: F, t6802: F, t6806: F, t6813: F) -> (F, F, F, F, F, F, F, F) {
        let (t23698, t23701, t23705, t23707, t23712, t23714) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1860::<F>(t23696, t23697, t23661, t3188, t1945, t3120, t1060, t23571, t383, t23384, t6787, t2776, t6785);
        let (t23715, t23720) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1861::<F>(t23714, t6784, t1003, t1058, t1953, t23346, t23601, t23666, t23670, t23674, t23680, t23687, t23693, t23698, t23701, t23705, t23707, t23712, t3076, t3186, t353, t6680, t6687, t6787, t6790, t6797, t6802, t6806, t6813);
    (t23698, t23701, t23705, t23707, t23712, t23714, t23715, t23720)
}
