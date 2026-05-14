//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 951/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk951<F: Float>(t6384: F, t8939: F, t904: F, t3258: F, t6390: F, t2255: F, t2313: F, t3111: F, t1158: F, t6505: F, t8574: F, t933: F, t2312: F, t2343: F, t9175: F, t9177: F, t929: F, t9626: F, t9632: F, t9634: F, t9637: F, t9641: F, t9645: F) -> (F, F, F, F, F, F) {
    let t9647 = t6384 * t904 * t8939;
    let t9650 = t3258 * t6390;
    let t9651 = t2255 * t9650;
    let t9655 = t2255 * t3111 * t2313;
    let t9658 = t6505 * t1158;
    let t9661 = t933 * t904 * t8574;
    let t9664 = t2343 * t9626 / 384.0 - t9632 - t2343 * t9634 / 1536.0 + t9637 * t9641 / 128.0 + t9175 - t9645 - 5.0 / 128.0 * t929 * t9647 - t9177 + t2312 * t9651 / 192.0 - t2312 * t9655 / 192.0 - 119.0 / 3456.0 * t9658 - t929 * t9661 / 768.0;
    (t9647, t9650, t9651, t9655, t9661, t9664)
}
