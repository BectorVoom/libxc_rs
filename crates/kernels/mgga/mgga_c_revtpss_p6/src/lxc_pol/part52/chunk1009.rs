//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1009/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1009<F: Float>(t28699: F, t28729: F, t28759: F, t28942: F, t3: F, t2055: F, t670: F, t1518: F, t26733: F, t4292: F, t7553: F, t116: F, t7983: F, param_d: F) -> (F, F, F, F, F, F, F) {
    let t28944 = t28699 + t28729 + t28759 + t28942;
    let t28945 = t3 * t28944;
    let t28956 = param_d * t28944;
    let t28974 = t670 * t2055;
    let t28975 = t28974 * t1518;
    let t28978 = t26733 * t1518;
    let t28981 = t7553 * t4292;
    let t28986 = t116 * t7983;
    (t28945, t28956, t28974, t28975, t28978, t28981, t28986)
}
