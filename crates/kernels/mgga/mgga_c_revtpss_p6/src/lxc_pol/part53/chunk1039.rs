//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1039/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1039<F: Float>(t1936: F, t27060: F, t29432: F, t7002: F, t7586: F, t32165: F, t32167: F, t32169: F, t32172: F, t32174: F, t32176: F, t32178: F, t32815: F, t32825: F, t670: F, t8564: F) -> F {
    let t32828 = t27060 * t1936;
    let t32830 = t29432 * t1936;
    let t32832 = t7586 * t7002;
    let t32837 = F::new(2.0) * t32825 * t670 + F::new(2.0) * t32165 + F::new(2.0) * t32167 + F::new(2.0) * t32169 + t32172 + t32174 + t32176 + t32178 + t32815 + F::new(2.0) * t32828 + F::new(2.0) * t32830 + F::new(2.0) * t32832 + t8564;
    t32837
}
