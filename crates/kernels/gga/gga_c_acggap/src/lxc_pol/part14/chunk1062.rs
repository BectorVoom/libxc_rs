//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1062/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1062<F: Float>(t1165: F, t38766: F, t7351: F, t7413: F, t1410: F, t8790: F, t604: F, t30037: F, t33860: F, t33865: F, t36825: F, t36828: F, t36829: F, t38732: F, t38736: F, t38740: F, t38743: F, t38747: F, t38751: F, t38755: F, t38757: F, t38760: F, t38764: F) -> (F, F) {
    let t38769 = t7413 * t1165 * t7351 * t38766;
    let t38771 = t8790 * t1410;
    let t38774 = t7413 * t1165 * t604 * t38771;
    let t38776 = -F::new(0.2250885951198661191e-1) * t38732 - F::new(0.64311027177104605458e-2) * t38736 + F::new(0.64311027177104605458e-2) * t38740 + t33860 - t36825 + F::new(0.51448821741683684367e-2) * t33865 - t36828 - F::new(0.10289764348336736873e-1) * t38743 + t36829 - F::new(0.94344276868812456204e-2) * t38747 + F::new(0.14151641530321868431e-1) * t38751 - F::new(0.64311027177104605458e-3) * t38755 - F::new(0.11321313224257494745e-1) * t38757 - F::new(0.34299214494455789578e-2) * t30037 + F::new(0.51448821741683684366e-2) * t38760 + F::new(0.53592522647587171215e-3) * t38764 + F::new(0.94344276868812456204e-3) * t38769 - F::new(0.62896184579208304136e-3) * t38774;
    (t38771, t38776)
}
