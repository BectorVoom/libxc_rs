//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 706/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk706<F: Float>(t11921: F, t828: F, t1035: F, t11239: F, t3143: F, t1043: F, t3153: F, t4171: F, t602: F, t1466: F, t2246: F) -> (F, F, F, F, F, F) {
    let t11922 = t828 * t11921;
    let t12046 = t11239 * t1035;
    let t12077 = t11239 * t3143;
    let t12131 = t1043 * t3153;
    let t13269 = t4171 * t602;
    let t13272 = t1466 * t2246;
    (t11922, t12046, t12077, t12131, t13269, t13272)
}
