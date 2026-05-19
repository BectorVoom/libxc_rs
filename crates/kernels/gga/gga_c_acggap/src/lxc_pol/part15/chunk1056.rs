//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1056/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1056<F: Float>(t35623: F, t35631: F, t35646: F, t35672: F, t35678: F, t35682: F, t35685: F, t35702: F, t35709: F, t35736: F, t35747: F, t35755: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37636 = F::cast_from(0.12579236915841660828e-2_f64) * t35623;
    let t37639 = F::cast_from(0.18868855373762491241e-2_f64) * t35631;
    let t37646 = F::new(0.305625e-1) * t35646;
    let t37658 = F::cast_from(0.13719685797782315831e-1_f64) * t35672;
    let t37661 = F::cast_from(0.13719685797782315831e-1_f64) * t35678;
    let t37663 = F::cast_from(0.57165357490759649296e-3_f64) * t35682;
    let t37665 = F::new(11.0) / F::new(24.0) * t35685;
    let t37672 = F::cast_from(0.18868855373762491241e-2_f64) * t35702;
    let t37675 = F::cast_from(0.64025200389650807212e-1_f64) * t35709;
    let t37696 = F::cast_from(0.68598428988911579156e-2_f64) * t35736;
    let t37701 = F::cast_from(0.85748036236139473944e-3_f64) * t35747;
    let t37704 = F::cast_from(0.34299214494455789578e-1_f64) * t35755;
    (t37636, t37639, t37646, t37658, t37661, t37663, t37665, t37672, t37675, t37696, t37701, t37704)
}
