//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 956/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk956<F: Float>(t1980: F, t38889: F, t7476: F, t1089: F, t2090: F, t22705: F, t598: F, t34406: F, t5928: F, t30817: F, t9645: F, t1849: F, t1983: F, t7380: F, t4680: F, t7575: F, t9669: F) -> (F, F, F, F, F, F) {
    let t38929 = t1980 * t7476 * t38889;
    let t38934 = t598 * t1089 * t22705 * t2090;
    let t38937 = t34406 * t5928;
    let t38939 = t30817 * t9645;
    let t38942 = t7380 * t1983 * t1849;
    let t38946 = t7575 * t4680 * t9669;
    (t38929, t38934, t38937, t38939, t38942, t38946)
}
