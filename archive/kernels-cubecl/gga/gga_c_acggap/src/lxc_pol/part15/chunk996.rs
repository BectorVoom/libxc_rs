//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 996/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk996<F: Float>(t30924: F, t30928: F, t1164: F, t8853: F, t31142: F, t8884: F, t2019: F, t8887: F, t8889: F, t30978: F, t30982: F, t30985: F) -> (F, F, F, F, F, F, F, F) {
    let t35123 = F::cast_from(0.75475421495049964964e-2_f64) * t30924;
    let t35125 = F::cast_from(0.75475421495049964964e-2_f64) * t30928;
    let t35137 = t1164 * t8853;
    let t35145 = t31142 * t8884;
    let t35148 = t2019 * t8887 * t8889;
    let t35160 = F::cast_from(0.16006300097412701803e-1_f64) * t30978;
    let t35162 = F::cast_from(0.16006300097412701803e-1_f64) * t30982;
    let t35163 = F::cast_from(0.21437009059034868486e-2_f64) * t30985;
    (t35123, t35125, t35137, t35145, t35148, t35160, t35162, t35163)
}
