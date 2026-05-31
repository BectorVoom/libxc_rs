//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1101/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1101<F: Float>(t1868: F, t2033: F, t26405: F, t25082: F, t1936: F, t28653: F, t34251: F, t7359: F, t7741: F, t2055: F, t34258: F, t93: F) -> (F, F, F, F, F, F, F, F) {
    let t34301 = t2033 * t1868;
    let t34302 = t26405 * t34301;
    let t34304 = F::cast_from(3.0_f64) * t25082 * t34302;
    let t34308 = F::cast_from(2.0_f64) * t28653 * t1936;
    let t34310 = F::cast_from(2.0_f64) * t34251 * t1936;
    let t34312 = F::cast_from(2.0_f64) * t7359 * t7741;
    let t34320 = F::cast_from(2.0_f64) * t34258 * t2055;
    let t34321 = t93 * t7741;
    (t34301, t34302, t34304, t34308, t34310, t34312, t34320, t34321)
}
