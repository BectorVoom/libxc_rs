//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1103/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1103<F: Float>(t1164: F, t8853: F, t2068: F, t2069: F, t31142: F, t8884: F, t2019: F, t8887: F, t8889: F, t142: F, t5183: F, t7436: F) -> (F, F, F, F) {
    let t35137 = t1164 * t8853;
    let t35139 = t2068 * t35137 * t2069;
    let t35145 = t31142 * t8884;
    let t35146 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t35145;
    let t35148 = t2019 * t8887 * t8889;
    let t35149 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t35148;
    let t35151 = t7436 * t142 * t5183;
    (t35139, t35146, t35149, t35151)
}
