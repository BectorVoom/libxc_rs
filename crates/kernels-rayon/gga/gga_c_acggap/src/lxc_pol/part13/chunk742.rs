//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 742/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk742(t1985: f64, t7799: f64, t606: f64, t7610: f64, t7748: f64, t7749: f64, t7751: f64, t7755: f64, t7756: f64, t7759: f64, t7761: f64, t7764: f64, t7768: f64, t7772: f64, t7774: f64, t7776: f64, t7782: f64, t7785: f64, t7788: f64, t7790: f64, t7793: f64, t7798: f64) -> (f64, f64, f64) {
    let t7800 = t7799 * t1985;
    let t7801 = 0.14291339372689912324e-3_f64 * t7800;
    let t7802 = t7610 * t606;
    let t7803 = 0.15724046144802076034e-3_f64 * t7802;
    let t7804 = t7748 + 0.25724410870841842183e-2_f64 * t7749 + 0.85748036236139473945e-2_f64 * t7751 - t7755 + 0.64311027177104605458e-2_f64 * t7756 + t7759 - t7761 - 0.10718504529517434243e-2_f64 * t7764 - 0.53592522647587171215e-3_f64 * t7768 - t7772 - t7774 - t7776 + t7782 - 0.31448092289604152068e-3_f64 * t7785 - t7788 + t7790 + 0.10718504529517434243e-3_f64 * t7793 + t7798 + t7801 - t7803;
    (t7801, t7803, t7804)
}
