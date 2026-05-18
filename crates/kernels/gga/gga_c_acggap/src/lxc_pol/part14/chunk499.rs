//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 499/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk499<F: Float>(t257: F, t2754: F, t249: F, t743: F, t62: F, t70: F, t746: F, t2742: F, t67: F, t747: F, t685: F, t80: F) -> (F, F, F, F, F, F) {
    let t2755 = t2754 * t257;
    let t2759 = F::new(1.0) / t743 / t249;
    let t2760 = t62 * t2759;
    let t2762 = F::new(1.0) / t746 / t70;
    let t2763 = t2742 * t2762;
    let t2767 = F::new(1.0) / t743 / t67;
    let t2768 = t62 * t2767;
    let t2769 = t2742 * t747;
    let t2773 = F::new(1.0) / t685 / t80;
    (t2755, t2760, t2763, t2768, t2769, t2773)
}
