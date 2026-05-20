//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta353 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1212;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1213;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1214;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta353<F: Float>(t45: F, t11064: F, t1583: F, t1469: F, t2609: F, t706: F, t10593: F, t10597: F, t4186: F, t80: F, t13312: F, t1490: F, t2251: F, t2258: F, t4328: F, t606: F, t766: F, zeta_threshold: F, t57: F, t83: F, t1491: F, t4335: F, t770: F, t1568: F, t785: F, t780: F, t2439: F, t212: F, t4469: F, t689: F, t1579: F, t2769: F, t886: F, t252: F, t2782: F, t2470: F, t4480: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t14436, t14442, t14443, t14444, t14455) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1212::<F>(t45, t11064, t1583, t1469, t2609, t706, t10593, t10597, t4186, t80, t13312, t1490, t2251, t2258, t4328, t606, t766, zeta_threshold);
        let (t14468, t14472) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1213::<F>(t57, t4186, t83, t13312, t1491, t2251, t2258, t4335, t606, t770, t14455, t1568, t785, zeta_threshold);
        let (t14474, t14479, t14484, t14485) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1214::<F>(t14472, t780, t2439, t212, t4469, t689, t1579, t2769, t886, t252, t2782, t2470, t4480);
    (t14436, t14442, t14443, t14444, t14468, t14474, t14479, t14484, t14485)
}
