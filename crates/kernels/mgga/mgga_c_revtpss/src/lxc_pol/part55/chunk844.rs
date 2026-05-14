//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 844/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk844<F: Float>(t2322: F, t7741: F, t5523: F, t1312: F, t28042: F, t2042: F, t5795: F, t1916: F, t7331: F, t7334: F, t1459: F, t7950: F, t1936: F, t670: F, t1518: F, t572: F) -> (F, F, F, F, F, F, F, F, F) {
    let t28225 = 2.0 * t2322 * t7741;
    let t28227 = 2.0 * t5523 * t7741;
    let t28229 = 2.0 * t1312 * t28042;
    let t28257 = 3.0 * t5795 * t2042;
    let t28259 = 6.0 * t1916 * t7331;
    let t28261 = 3.0 * t1916 * t7334;
    let t28263 = 6.0 * t1459 * t7950;
    let t28264 = t670 * t1936;
    let t28265 = t28264 * t1518;
    let t28267 = 6.0 * t572 * t28265;
    (t28225, t28227, t28229, t28257, t28259, t28261, t28263, t28265, t28267)
}
