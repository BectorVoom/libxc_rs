//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 992/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk992<F: Float>(t35113: F, t1164: F, t8853: F, t31142: F, t8884: F, t2019: F, t8887: F, t8889: F, t1992: F, t30127: F, t7842: F, t8791: F) -> (F, F, F, F, F) {
    let t35114 = F::cast_from(0.94344276868812456204e-2_f64) * t35113;
    let t35137 = t1164 * t8853;
    let t35145 = t31142 * t8884;
    let t35146 = F::new(7.0) / F::new(72.0) * t35145;
    let t35148 = t2019 * t8887 * t8889;
    let t35149 = F::new(7.0) / F::new(72.0) * t35148;
    let t35176 = t30127 * t7842 * t1992 * t8791;
    (t35114, t35137, t35146, t35149, t35176)
}
