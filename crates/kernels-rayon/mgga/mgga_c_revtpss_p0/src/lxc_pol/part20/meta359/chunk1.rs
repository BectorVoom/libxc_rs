//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1306/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1306(t10657: f64, t2646: f64, t2724: f64, t39622: f64, t39624: f64, t39629: f64, t39633: f64, t39635: f64, t39640: f64, t39649: f64, t39652: f64, t39656: f64, t39662: f64, t820: f64) -> f64 {
    let t39664 = 0.65854491829355115985e-1_f64 * t39622 - 0.44178176337912614788e-3_f64 * t39624 + 0.23417857294518679245e0_f64 * t39629 + t39633 + 0.12142592671231907757e0_f64 * t39635 - 0.69394917116090352835e-2_f64 * t39640 + t39649 - t39652 - 0.39512695097613069592e1_f64 * t820 * t10657 * t2646 + 0.79025390195226139183e1_f64 * t820 * t39656 * t2724 - 0.1561190486301245283e0_f64 * t39662;
    t39664
}
