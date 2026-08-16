//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta614 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2519;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2520;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta614<F: Float>(t19644: F, t3092: F, t1065: F, t6244: F, t906: F, t1042: F, t3172: F, t6301: F, t1041: F, t5819: F, t606: F, t16199: F, t1469: F, t4186: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t19645, t19649, t19650, t19651, t19658, t19659, t19661) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2519::<F>(t19644, t3092, t1065, t6244, t906, t1042, t3172, t6301, t1041, t5819, t606);
        let (t19662, t19663, t19666) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2520::<F>(t16199, t19661, t1042, t1469, t4186);
    (t19645, t19649, t19650, t19651, t19658, t19659, t19661, t19662, t19663, t19666)
}
