//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 952/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk952<F: Float>(t1936: F, t27123: F, t4292: F, t93: F, t7002: F, t7889: F, t2322: F, t7741: F, t5523: F, t1312: F, t28042: F, t1518: F, t25805: F, t28025: F, t28030: F, t28160: F, t28212: F, t28214: F, t28216: F, t670: F, t6985: F) -> (F, F, F, F, F, F, F, F) {
    let t28218 = F::new(2.0) * t27123 * t1936;
    let t28219 = t93 * t4292;
    let t28221 = F::new(2.0) * t28219 * t1936;
    let t28223 = F::new(2.0) * t7889 * t7002;
    let t28225 = F::new(2.0) * t2322 * t7741;
    let t28227 = F::new(2.0) * t5523 * t7741;
    let t28229 = F::new(2.0) * t1312 * t28042;
    let t28230 = F::new(2.0) * t1518 * t25805 + F::new(2.0) * t1518 * t28025 + F::new(2.0) * t28030 * t670 + F::new(2.0) * t4292 * t6985 + t28160 + t28212 + t28214 + t28216 + t28218 + t28221 + t28223 + t28225 + t28227 + t28229;
    (t28218, t28219, t28221, t28223, t28225, t28227, t28229, t28230)
}
