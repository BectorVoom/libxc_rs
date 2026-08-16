//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta443 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1667;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1668;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta443(t141: f64, t16886: f64, t1145: f64, t16733: f64, t5098: f64, t698: f64, t16725: f64, t3417: f64, t16729: f64, t16720: f64, t16738: f64, t12254: f64, t16715: f64, t16708: f64, t16710: f64, t16712: f64, t12296: f64, t12297: f64, t12299: f64, t12301: f64, t12303: f64, t16706: f64, t16717: f64, t16722: f64, t16727: f64, t16731: f64, t16735: f64, t16740: f64, t16744: f64, t16748: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16887, t16890, t16892, t16893, t16895, t16898, t16901, t16904, t16907) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1667(t141, t16886, t1145, t16733, t5098, t698, t16725, t3417, t16729, t16720, t16738, t12254, t16715);
        let (t16908, t16926) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1668(t141, t16907, t16708, t16710, t16712, t12296, t12297, t12299, t12301, t12303, t16706, t16717, t16722, t16727, t16731, t16735, t16740, t16744, t16748);
    (t16887, t16890, t16892, t16893, t16895, t16898, t16901, t16904, t16908, t16926)
}
