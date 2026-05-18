//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 854/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk854<F: Float>(t26028: F, t5701: F, t5706: F, t5614: F, t7271: F, t5661: F, t7264: F, t25997: F, t5665: F, t1873: F, t26004: F, t5690: F, t7252: F) -> (F, F, F, F, F, F, F) {
    let t27945 = t26028 * t5701;
    let t27947 = t26028 * t5706;
    let t27949 = t7271 * t5614;
    let t27951 = t7264 * t5661;
    let t27953 = t25997 * t5665;
    let t27955 = t26004 * t1873;
    let t27957 = t7252 * t5690;
    (t27945, t27947, t27949, t27951, t27953, t27955, t27957)
}
