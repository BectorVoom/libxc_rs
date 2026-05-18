//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1259/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1259<F: Float>(t10914: F, t10915: F, t32803: F, t1: F, t106: F, t5745: F, t787: F, t191: F, t5750: F, t24784: F, t2660: F, t10827: F, t2684: F, t7354: F) -> (F, F, F, F) {
    let t32806 = F::new(0.21450293971110256001e2) * t10914 * t10915 * t32803;
    let t32809 = t787 * t5745 * t1 * t106;
    let t32810 = t191 * t5750;
    let t32813 = F::new(0.85801175884441024004e1) * t32809 * t32810 * t32803;
    let t32815 = F::new(0.21450293971110256002e1) * t24784 * t2660;
    let t32817 = t2684 * t7354 * t10827;
    (t32806, t32813, t32815, t32817)
}
