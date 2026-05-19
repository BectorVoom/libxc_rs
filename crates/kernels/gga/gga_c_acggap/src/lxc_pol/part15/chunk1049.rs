//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1049/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1049<F: Float>(t34570: F, t34592: F, t34609: F, t34618: F, t34620: F, t34626: F, t34632: F, t34659: F, t34702: F, t34704: F, t34710: F, t34712: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37158 = F::cast_from(0.12862205435420921092e-1_f64) * t34570;
    let t37167 = F::new(11.0) / F::new(96.0) * t34592;
    let t37175 = F::new(11.0) / F::new(96.0) * t34609;
    let t37179 = F::cast_from(0.2264262644851498949e-1_f64) * t34618;
    let t37180 = F::cast_from(0.37737710747524982482e-2_f64) * t34620;
    let t37182 = F::cast_from(0.18868855373762491241e-2_f64) * t34626;
    let t37184 = F::cast_from(0.37737710747524982482e-1_f64) * t34632;
    let t37197 = F::new(7.0) / F::new(36.0) * t34659;
    let t37213 = F::cast_from(0.25724410870841842184e-1_f64) * t34702;
    let t37214 = F::cast_from(0.1543464652250510531e-1_f64) * t34704;
    let t37216 = F::cast_from(0.25724410870841842184e-2_f64) * t34710;
    let t37217 = F::cast_from(0.25724410870841842184e-2_f64) * t34712;
    (t37158, t37167, t37175, t37179, t37180, t37182, t37184, t37197, t37213, t37214, t37216, t37217)
}
