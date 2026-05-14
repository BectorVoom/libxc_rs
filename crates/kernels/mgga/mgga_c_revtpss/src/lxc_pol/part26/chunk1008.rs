//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1008/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1008<F: Float>(t3981: F, t94443: F, t7271: F, t9944: F, t25986: F, t2661: F, t9930: F, t9757: F, t25981: F, t820: F, t843: F, t4006: F, t2681: F, t7262: F, t1401: F, t7264: F, t9901: F) -> (F, F, F, F, F, F, F) {
    let t94444 = t94443 * t3981;
    let t94446 = t7271 * t9944;
    let t94449 = t2661 * t25986 * t9930;
    let t94451 = t7271 * t9757;
    let t94455 = t820 * t25981 * t843;
    let t94456 = t94455 * t4006;
    let t94459 = t820 * t7262 * t2681;
    let t94460 = t94459 * t1401;
    let t94462 = t7264 * t9901;
    (t94444, t94446, t94449, t94451, t94456, t94460, t94462)
}
