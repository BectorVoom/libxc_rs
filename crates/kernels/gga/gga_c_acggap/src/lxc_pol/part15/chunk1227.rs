//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1227/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1227<F: Float>(t32543: F, t32544: F, t32545: F, t32557: F, t34655: F, t34675: F, t34698: F, t34718: F, t34722: F, t34724: F, t34738: F, t34743: F, t37197: F, t37213: F, t37214: F, t37216: F, t37217: F, t39427: F) -> F {
    let t41646 = -t32543 - t32544 + t32545 - t34655 + t37197 + F::cast_from(0.83861579438944405517e-3_f64) * t34675 - F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t39427 - F::cast_from(0.85748036236139473944e-2_f64) * t34698 - t37213 - t37214 + t37216 + t37217 - F::cast_from(0.10289764348336736873e-1_f64) * t34718 + F::cast_from(0.62896184579208304136e-2_f64) * t34722 + F::cast_from(0.37737710747524982482e-1_f64) * t34724 + F::cast_from(0.51448821741683684367e-2_f64) * t34738 - t32557 - t34743;
    t41646
}
