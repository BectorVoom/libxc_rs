//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1039/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1039<F: Float>(t1445: F, t3209: F, t813: F, t8528: F, t10915: F, t22242: F, t43598: F, t2684: F, t43486: F, t7585: F, t10930: F, t10931: F) -> (F, F, F, F) {
    let t43787 = F::new(0.92023022289409799224e1) * t813 * t1445 * t8528 * t3209;
    let t43790 = F::new(0.21450293971110256001e1) * t22242 * t10915 * t43598;
    let t43793 = F::new(0.87421871174939309262e2) * t2684 * t7585 * t43486;
    let t43800 = F::new(0.55213813373645879534e2) * t10930 * t10931 * t43486;
    (t43787, t43790, t43793, t43800)
}
