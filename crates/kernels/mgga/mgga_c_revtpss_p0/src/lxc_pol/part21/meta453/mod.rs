//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta453 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1982;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1983;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta453<F: Float>(t45: F, t11064: F, t1583: F, t1469: F, t2609: F, t706: F, t10593: F, t10597: F, t4186: F, t80: F, t13312: F, t1490: F, t2251: F, t2258: F, t4328: F, t606: F, t766: F, zeta_threshold: F, t57: F, t83: F, t1491: F, t4335: F, t770: F) -> (F, F, F, F, F, F, F, F) {
        let (t14436, t14440, t14442, t14443, t14444, t14447, t14455) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1982::<F>(t45, t11064, t1583, t1469, t2609, t706, t10593, t10597, t4186, t80, t13312, t1490, t2251, t2258, t4328, t606, t766, zeta_threshold);
        let (t14458, t14468) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1983::<F>(t57, t4186, t83, t13312, t1491, t2251, t2258, t4335, t606, t770, t14455, zeta_threshold);
    (t14436, t14440, t14442, t14443, t14444, t14447, t14458, t14468)
}
