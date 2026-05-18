//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1062/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1062<F: Float>(t1158: F, t6505: F, t8574: F, t904: F, t933: F, t2312: F, t2343: F, t9175: F, t9177: F, t929: F, t9626: F, t9632: F, t9634: F, t9637: F, t9641: F, t9645: F, t9647: F, t9651: F, t9655: F) -> (F, F) {
    let t9658 = t6505 * t1158;
    let t9661 = t933 * t904 * t8574;
    let t9664 = t2343 * t9626 / F::new(384.0) - t9632 - t2343 * t9634 / F::new(1536.0) + t9637 * t9641 / F::new(128.0) + t9175 - t9645 - F::new(5.0) / F::new(128.0) * t929 * t9647 - t9177 + t2312 * t9651 / F::new(192.0) - t2312 * t9655 / F::new(192.0) - F::new(119.0) / F::new(3456.0) * t9658 - t929 * t9661 / F::new(768.0);
    (t9661, t9664)
}
