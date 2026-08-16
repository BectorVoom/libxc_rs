//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1062/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1062(t1165: f64, t38766: f64, t7351: f64, t7413: f64, t1410: f64, t8790: f64, t604: f64, t30037: f64, t33860: f64, t33865: f64, t36825: f64, t36828: f64, t36829: f64, t38732: f64, t38736: f64, t38740: f64, t38743: f64, t38747: f64, t38751: f64, t38755: f64, t38757: f64, t38760: f64, t38764: f64) -> (f64, f64) {
    let t38769 = t7413 * t1165 * t7351 * t38766;
    let t38771 = t8790 * t1410;
    let t38774 = t7413 * t1165 * t604 * t38771;
    let t38776 = -0.2250885951198661191e-1_f64 * t38732 - 0.64311027177104605458e-2_f64 * t38736 + 0.64311027177104605458e-2_f64 * t38740 + t33860 - t36825 + 0.51448821741683684367e-2_f64 * t33865 - t36828 - 0.10289764348336736873e-1_f64 * t38743 + t36829 - 0.94344276868812456204e-2_f64 * t38747 + 0.14151641530321868431e-1_f64 * t38751 - 0.64311027177104605458e-3_f64 * t38755 - 0.11321313224257494745e-1_f64 * t38757 - 0.34299214494455789578e-2_f64 * t30037 + 0.51448821741683684366e-2_f64 * t38760 + 0.53592522647587171215e-3_f64 * t38764 + 0.94344276868812456204e-3_f64 * t38769 - 0.62896184579208304136e-3_f64 * t38774;
    (t38771, t38776)
}
