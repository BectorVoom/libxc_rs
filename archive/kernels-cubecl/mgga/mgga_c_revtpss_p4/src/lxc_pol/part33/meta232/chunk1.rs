//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1052/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1052<F: Float>(t6244: F, t996: F, t1651: F, t1695: F, t1079: F, t3070: F, t4571: F, t6094: F, t6098: F, t6102: F) -> (F, F, F) {
    let t6245 = t996 * t6244;
    let t6250 = t1651 * t1695;
    let t6251 = t1079 * t6250;
    let t6258 = t3070 + F::cast_from(0.9877777777777777778e-2_f64) * t4571 - F::cast_from(0.9877777777777777778e-2_f64) * t6094 + F::cast_from(0.29633333333333333334e-1_f64) * t6098 - F::cast_from(0.14816666666666666667e-1_f64) * t6102;
    (t6245, t6251, t6258)
}
