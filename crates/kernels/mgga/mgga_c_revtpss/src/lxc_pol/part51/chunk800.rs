//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 800/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk800<F: Float>(t2033: F, t9593: F, t1448: F, t1907: F, t28196: F, t7316: F, t7898: F, t13426: F, t1936: F, t18227: F, t4248: F, t7002: F, t27123: F, t4292: F, t93: F, t7889: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t28197 = t2033 * t9593;
    let t28198 = t1907 * t1448;
    let t28199 = t28197 * t28198;
    let t28201 = 2.0 * t28196 * t28199;
    let t28202 = t7898 * t7316;
    let t28212 = 2.0 * t13426 * t1936;
    let t28214 = 2.0 * t18227 * t1936;
    let t28216 = 2.0 * t4248 * t7002;
    let t28218 = 2.0 * t27123 * t1936;
    let t28219 = t93 * t4292;
    let t28221 = 2.0 * t28219 * t1936;
    let t28223 = 2.0 * t7889 * t7002;
    (t28197, t28198, t28199, t28201, t28202, t28212, t28214, t28216, t28218, t28219, t28221, t28223)
}
