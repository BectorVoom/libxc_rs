//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1267/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1267<F: Float>(t36385: F, t36386: F, t36390: F, t36392: F, t37987: F, t37988: F, t37992: F, t40553: F, t40557: F, t40561: F, t40565: F, t40567: F, t40569: F, t40573: F, t40576: F, t40579: F, t40584: F, t40587: F) -> F {
    let t42175 = F::new(0.37737710747524982482e-2) * t40553 + F::new(0.37737710747524982482e-2) * t40557 + F::new(0.37737710747524982482e-2) * t40561 + F::new(0.25158473831683321655e-2) * t40565 + F::new(0.37737710747524982483e-2) * t40567 - F::new(0.183375e0) * t40569 + t37987 - t40573 / F::new(192.0) + t37988 + F::new(0.1528125e-1) * t40576 + t40579 / F::new(16.0) - t36385 + F::new(0.11181543925192587402e-1) * t36386 + t37992 + F::new(0.75475421495049964965e-2) * t36390 + F::new(0.68598428988911579156e-2) * t36392 + F::new(0.62896184579208304136e-2) * t40584 + F::new(0.37737710747524982482e-2) * t40587;
    t42175
}
