//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1011/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1011<F: Float>(t4021: F, t94497: F, t25986: F, t2661: F, t9980: F, t26024: F, t3926: F, t4059: F, t2482: F, t25981: F, t27: F, t10003: F, t25997: F, t9970: F, t550: F, t7021: F) -> (F, F, F, F, F, F, F) {
    let t94498 = t94497 * t4021;
    let t94501 = t2661 * t25986 * t9980;
    let t94503 = t26024 * t3926;
    let t94505 = t26024 * t4059;
    let t94508 = t2482 * t25981 * t27;
    let t94509 = t94508 * t10003;
    let t94511 = t25997 * t9970;
    let t94513 = t7021 * t550;
    (t94498, t94501, t94503, t94505, t94509, t94511, t94513)
}
