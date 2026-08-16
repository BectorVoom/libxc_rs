//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta513 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1835;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta513(t10867: f64, t64: f64, t2681: f64, t7043: f64, t820: f64, t857: f64, t25222: f64, t2656: f64, t2482: f64, t596: f64, t2677: f64, t10741: f64, t25234: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t93060, t93066, t93067, t93069, t93072, t93073, t93077) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1835(t10867, t64, t2681, t7043, t820, t857, t25222, t2656, t2482, t596, t2677, t10741, t25234);
    (t93060, t93066, t93067, t93069, t93072, t93073, t93077)
}
