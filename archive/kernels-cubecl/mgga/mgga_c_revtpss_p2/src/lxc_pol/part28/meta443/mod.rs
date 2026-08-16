//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta443 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1667;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1668;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta443<F: Float>(t141: F, t16886: F, t1145: F, t16733: F, t5098: F, t698: F, t16725: F, t3417: F, t16729: F, t16720: F, t16738: F, t12254: F, t16715: F, t16708: F, t16710: F, t16712: F, t12296: F, t12297: F, t12299: F, t12301: F, t12303: F, t16706: F, t16717: F, t16722: F, t16727: F, t16731: F, t16735: F, t16740: F, t16744: F, t16748: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t16887, t16890, t16892, t16893, t16895, t16898, t16901, t16904, t16907) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1667::<F>(t141, t16886, t1145, t16733, t5098, t698, t16725, t3417, t16729, t16720, t16738, t12254, t16715);
        let (t16908, t16926) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1668::<F>(t141, t16907, t16708, t16710, t16712, t12296, t12297, t12299, t12301, t12303, t16706, t16717, t16722, t16727, t16731, t16735, t16740, t16744, t16748);
    (t16887, t16890, t16892, t16893, t16895, t16898, t16901, t16904, t16908, t16926)
}
