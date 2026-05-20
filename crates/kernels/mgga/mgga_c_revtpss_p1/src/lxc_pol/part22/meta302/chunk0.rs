//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1735/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1735<F: Float>(t10039: F, t869: F, t689: F, t2777: F, t4092: F, t2439: F, t1419: F, t3999: F, t3923: F, t555: F, t4003: F, t5744: F) -> (F, F, F, F, F, F, F) {
    let t10040 = t869 * t10039;
    let t10041 = t689 * t10040;
    let t10043 = t2777 * t4092;
    let t10044 = t2439 * t10043;
    let t10049 = t3999 * t1419;
    let t10059 = t555 * t3923;
    let t10061 = t5744 * t10059 * t4003;
    (t10040, t10041, t10043, t10044, t10049, t10059, t10061)
}
