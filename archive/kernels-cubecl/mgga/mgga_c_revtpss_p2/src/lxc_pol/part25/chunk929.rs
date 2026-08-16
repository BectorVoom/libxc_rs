//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 929/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk929<F: Float>(t2440: F, t887: F, t2439: F, t866: F, t225: F, t2771: F, t886: F, t2461: F, t2471: F, t788: F, t9288: F, t787: F) -> (F, F, F, F, F, F, F, F) {
    let t11003 = t2440 * t887;
    let t11004 = t2439 * t11003;
    let t11006 = t866 * t866;
    let t11007 = F::cast_from(1.0_f64) / t11006;
    let t11008 = t225 * t11007;
    let t11009 = t2771 * t886;
    let t11010 = t11008 * t11009;
    let t11013 = t2461 * t2471;
    let t11015 = t788 * t9288;
    let t11017 = F::cast_from(0.30356481678079769392e-1_f64) * t787 * t11015;
    (t11004, t11006, t11007, t11009, t11010, t11013, t11015, t11017)
}
