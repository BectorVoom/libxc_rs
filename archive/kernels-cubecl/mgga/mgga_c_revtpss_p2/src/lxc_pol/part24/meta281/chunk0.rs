//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1055/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1055<F: Float>(t2411: F, t6075: F, t11506: F, t6189: F, t11144: F, t5819: F, t11150: F, t6093: F, t689: F) -> (F, F, F, F, F) {
    let t18865 = t6075 * t2411;
    let t18898 = t11506 * t6189;
    let t18903 = t11144 * t5819;
    let t18908 = t11150 * t5819;
    let t18919 = t689 * t6093;
    (t18865, t18898, t18903, t18908, t18919)
}
