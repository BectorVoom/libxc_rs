//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1212/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1212<F: Float>(t19542: F, t20190: F, t1639: F, t520: F, t5918: F, t5745: F, t1838: F, t4459: F, t18967: F, t19554: F, t1265: F, t5740: F, t6419: F) -> (F, F, F, F, F) {
    let t20191 = t20190 * t19542;
    let t20195 = t5918 * t1639 * t520;
    let t20196 = t5745 * t20195;
    let t20200 = t5745 * t1838 * t4459 * t520;
    let t20202 = t18967 * t19554;
    let t20206 = t5740 * t6419 * t1265;
    (t20191, t20196, t20200, t20202, t20206)
}
