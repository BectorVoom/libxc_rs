//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1351/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1351<F: Float>(t231: F, t40250: F, t10639: F, t10657: F, t2754: F, t2815: F, t39707: F, t39712: F, t39714: F, t39719: F, t39723: F, t39724: F, t39726: F, t39731: F, t4514: F, t820: F, t837: F, t879: F) -> (F, F) {
    let t40251 = t40250 * t231;
    let t40255 = F::cast_from(0.65854491829355115985e-1_f64) * t39707 - F::cast_from(0.13170898365871023197e0_f64) * t39712 - F::cast_from(0.26341796731742046395e1_f64) * t4514 * t39714 * t837 + F::cast_from(0.78548797528808629095e-3_f64) * t39719 - t39723 + F::cast_from(0.1040793657534163522e-1_f64) * t39724 - F::cast_from(0.43902994552903410657e-1_f64) * t39726 - F::cast_from(0.26341796731742046395e1_f64) * t820 * t2815 * t10639 + F::cast_from(0.15611904863012452831e0_f64) * t39731 - F::cast_from(0.39512695097613069592e1_f64) * t820 * t10657 * t2754 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t879 * t40251;
    (t40251, t40255)
}
