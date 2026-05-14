//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 923/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk923<F: Float>(t1466: F, t30644: F, t13889: F, t2068: F, t2267: F, t1181: F, t4516: F, t7351: F, t7564: F, t137: F, t14423: F, t1165: F, t5012: F, t30209: F, t5099: F, t604: F) -> (F, F, F, F, F) {
    let t34239 = t30644 * t1466;
    let t34240 = 0.17149607247227894789e-2 * t34239;
    let t34242 = t2068 * t13889 * t2267;
    let t34246 = t7564 * t1181 * t7351 * t4516;
    let t34248 = t14423 * t137;
    let t34251 = t7564 * t1165 * t34248 * t5012;
    let t34255 = t30209 * t1181 * t604 * t5099;
    (t34240, t34242, t34246, t34251, t34255)
}
