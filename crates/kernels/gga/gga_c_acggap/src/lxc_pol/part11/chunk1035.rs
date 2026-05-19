//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1035/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1035<F: Float>(t1181: F, t4521: F, t604: F, t7426: F, t1466: F, t30644: F, t13889: F, t2068: F, t2267: F, t4516: F, t7351: F, t7564: F) -> (F, F, F, F) {
    let t34237 = t7426 * t1181 * t604 * t4521;
    let t34239 = t30644 * t1466;
    let t34240 = F::cast_from(0.17149607247227894789e-2_f64) * t34239;
    let t34242 = t2068 * t13889 * t2267;
    let t34246 = t7564 * t1181 * t7351 * t4516;
    (t34237, t34240, t34242, t34246)
}
