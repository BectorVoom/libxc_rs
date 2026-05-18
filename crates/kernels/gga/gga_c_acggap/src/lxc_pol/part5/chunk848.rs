//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 848/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk848<F: Float>(t11787: F, t219: F, t2809: F, t699: F, t709: F, t11805: F, t31: F, t4: F, t35: F, t595: F, t88: F, t11870: F, t2792: F, t286: F, t690: F) -> (F, F, F, F, F) {
    let t11909 = F::new(24.0) * t2809 * t11787 * t219;
    let t11910 = t709 * t699;
    let t11914 = F::new(0.11483599538271604938e-1) * t4 * t11805 * t31;
    let t11916 = t35 * t595 * t88;
    let t11921 = F::new(0.6233709278045326953e3) * t286 * t2792 * t11870 * t690;
    (t11909, t11910, t11914, t11916, t11921)
}
