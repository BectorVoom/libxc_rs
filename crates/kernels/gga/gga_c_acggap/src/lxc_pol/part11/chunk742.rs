//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 742/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk742<F: Float>(t1985: F, t7799: F, t606: F, t7610: F, t7748: F, t7749: F, t7751: F, t7755: F, t7756: F, t7759: F, t7761: F, t7764: F, t7768: F, t7772: F, t7774: F, t7776: F, t7782: F, t7785: F, t7788: F, t7790: F, t7793: F, t7798: F) -> (F, F, F) {
    let t7800 = t7799 * t1985;
    let t7801 = F::new(0.14291339372689912324e-3) * t7800;
    let t7802 = t7610 * t606;
    let t7803 = F::new(0.15724046144802076034e-3) * t7802;
    let t7804 = t7748 + F::new(0.25724410870841842183e-2) * t7749 + F::new(0.85748036236139473945e-2) * t7751 - t7755 + F::new(0.64311027177104605458e-2) * t7756 + t7759 - t7761 - F::new(0.10718504529517434243e-2) * t7764 - F::new(0.53592522647587171215e-3) * t7768 - t7772 - t7774 - t7776 + t7782 - F::new(0.31448092289604152068e-3) * t7785 - t7788 + t7790 + F::new(0.10718504529517434243e-3) * t7793 + t7798 + t7801 - t7803;
    (t7801, t7803, t7804)
}
