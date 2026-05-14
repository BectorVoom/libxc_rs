//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 857/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk857<F: Float>(t1936: F, t25805: F, t28025: F, t6985: F, t7002: F, t648: F, t8453: F, t8692: F, t2322: F, t8460: F, t5523: F, t32161: F, t32162: F, t670: F, t8564: F, t7235: F, t8596: F) -> (F, F, F, F, F) {
    let t32165 = t25805 * t1936;
    let t32167 = t28025 * t1936;
    let t32169 = t6985 * t7002;
    let t32171 = t648 * t8453;
    let t32172 = 2.0 * t32171;
    let t32174 = 4.0 * t8692 * t7002;
    let t32175 = t2322 * t8460;
    let t32176 = 2.0 * t32175;
    let t32177 = t5523 * t8460;
    let t32178 = 2.0 * t32177;
    let t32179 = 2.0 * t32162 * t670 + t32161 + 4.0 * t32165 + 4.0 * t32167 + 4.0 * t32169 + t32172 + t32174 + t32176 + t32178 + t8564;
    let t32182 = t7235 * t8596;
    (t32171, t32176, t32178, t32179, t32182)
}
