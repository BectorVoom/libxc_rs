//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1306/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1306<F: Float>(t10657: F, t2646: F, t2724: F, t39622: F, t39624: F, t39629: F, t39633: F, t39635: F, t39640: F, t39649: F, t39652: F, t39656: F, t39662: F, t820: F) -> F {
    let t39664 = F::cast_from(0.65854491829355115985e-1_f64) * t39622 - F::cast_from(0.44178176337912614788e-3_f64) * t39624 + F::cast_from(0.23417857294518679245e0_f64) * t39629 + t39633 + F::cast_from(0.12142592671231907757e0_f64) * t39635 - F::cast_from(0.69394917116090352835e-2_f64) * t39640 + t39649 - t39652 - F::cast_from(0.39512695097613069592e1_f64) * t820 * t10657 * t2646 + F::cast_from(0.79025390195226139183e1_f64) * t820 * t39656 * t2724 - F::cast_from(0.1561190486301245283e0_f64) * t39662;
    t39664
}
