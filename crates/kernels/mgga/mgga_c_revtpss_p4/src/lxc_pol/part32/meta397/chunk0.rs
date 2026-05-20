//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1373/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1373<F: Float>(t125: F, t5977: F, t10786: F, t2747: F, t221: F, t2485: F, t6022: F, t10850: F, t5962: F, t775: F) -> (F, F, F, F, F) {
    let t18426 = t125 * t5977;
    let t18428 = t2747 * t18426 * t10786;
    let t18432 = t2485 * t221 * t6022;
    let t18433 = t10850 * t18432;
    let t18435 = t5962 * t775;
    (t18426, t18428, t18432, t18433, t18435)
}
