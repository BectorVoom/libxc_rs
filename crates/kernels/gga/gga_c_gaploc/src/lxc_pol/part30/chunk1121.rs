//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1121/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1121<F: Float>(t10914: F, t10915: F, t32803: F, t1: F, t106: F, t5745: F, t787: F, t191: F, t5750: F, t24784: F, t2660: F, t10827: F, t2684: F, t7354: F, t2033: F, t2365: F, t27728: F) -> (F, F, F, F, F) {
    let t32806 = 0.21450293971110256001e2 * t10914 * t10915 * t32803;
    let t32809 = t787 * t5745 * t1 * t106;
    let t32810 = t191 * t5750;
    let t32813 = 0.85801175884441024004e1 * t32809 * t32810 * t32803;
    let t32815 = 0.21450293971110256002e1 * t24784 * t2660;
    let t32817 = t2684 * t7354 * t10827;
    let t32818 = 0.51123901271894332902e0 * t32817;
    let t32820 = t2033 * t2365 * t27728;
    (t32806, t32813, t32815, t32818, t32820)
}
