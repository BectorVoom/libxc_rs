//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 735/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk735<F: Float>(t1039: F, t7741: F, t7678: F, t7682: F, t7686: F, t7690: F, t7694: F, t7697: F, t7699: F, t7702: F, t7706: F, t7710: F, t7714: F, t7715: F, t7718: F, t7722: F, t7726: F, t7729: F, t7734: F, t7738: F, t7740: F) -> (F, F) {
    let t7742 = t7741 * t1039;
    let t7743 = F::cast_from(0.85748036236139473944e-3_f64) * t7742;
    let t7744 = t7678 - F::cast_from(0.18868855373762491241e-2_f64) * t7682 + F::cast_from(0.40015750243531754508e-2_f64) * t7686 + F::cast_from(0.32155513588552302729e-2_f64) * t7690 + F::cast_from(0.64311027177104605458e-3_f64) * t7694 - t7697 - t7699 - F::cast_from(0.42874018118069736972e-3_f64) * t7702 - F::cast_from(0.21437009059034868486e-3_f64) * t7706 + F::cast_from(0.62896184579208304135e-3_f64) * t7710 - t7714 - F::cast_from(0.42874018118069736972e-3_f64) * t7715 - t7718 - t7722 - t7726 - F::cast_from(0.10718504529517434243e-3_f64) * t7729 + F::cast_from(0.85748036236139473944e-3_f64) * t7734 - t7738 - t7740 + t7743;
    (t7743, t7744)
}
