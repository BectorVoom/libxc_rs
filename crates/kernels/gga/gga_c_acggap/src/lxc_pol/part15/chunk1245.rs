//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1245/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1245<F: Float>(t35486: F, t35499: F, t37566: F, t37567: F, t37569: F, t37573: F, t37576: F, t39985: F, t39987: F, t39990: F, t39995: F, t39999: F, t40003: F, t40005: F, t40009: F, t40011: F, t40015: F, t40019: F) -> F {
    let t41895 = t37566 + t37567 - t37569 + F::new(0.85748036236139473944e-3) * t39985 + F::new(0.12579236915841660828e-2) * t39987 + F::new(0.12579236915841660828e-2) * t39990 - F::new(0.51448821741683684367e-2) * t35486 + F::new(0.62896184579208304138e-3) * t39995 - F::new(0.18868855373762491241e-1) * t39999 - F::new(0.15095084299009992993e-1) * t40003 + F::new(0.11321313224257494745e-1) * t40005 - F::new(0.62896184579208304138e-3) * t40009 + t37573 - F::new(0.75475421495049964968e-2) * t40011 + F::new(0.62896184579208304138e-3) * t40015 - F::new(0.12862205435420921092e-1) * t40019 - t35499 - t37576;
    t41895
}
