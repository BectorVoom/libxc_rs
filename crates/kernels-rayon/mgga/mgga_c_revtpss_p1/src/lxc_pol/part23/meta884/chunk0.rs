//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2796/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2796(t22307: f64, t545: f64, t689: f64, t869: f64, t14239: f64, t14242: f64, t10023: f64, t22314: f64, t2470: f64, t13790: f64, t5658: f64, t10022: f64, t2782: f64) -> (f64, f64, f64, f64) {
    let t75174 = t689 * t869 * t545 * t22307;
    let t75176 = t14239 * t14242;
    let t75179 = t10023 * t22314 * t2470;
    let t75188 = t13790 * t5658;
    let t75190 = t2782 * t10022 * t75188;
    (t75174, t75176, t75179, t75190)
}
