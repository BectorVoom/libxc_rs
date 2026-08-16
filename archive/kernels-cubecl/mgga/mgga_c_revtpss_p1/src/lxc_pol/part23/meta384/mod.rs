//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta384 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1729;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta384<F: Float>(t16712: F, t12256: F, t1469: F, t3362: F, t4186: F, t3367: F, t3153: F, t5284: F, t300: F, t5155: F, t16710: F, t16708: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t16713, t16714, t16724, t16737, t16756, t16784, t16797, t16798, t16820) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1729::<F>(t16712, t12256, t1469, t3362, t4186, t3367, t3153, t5284, t300, t5155, t16710, t16708);
    (t16713, t16714, t16724, t16737, t16756, t16784, t16797, t16798, t16820)
}
