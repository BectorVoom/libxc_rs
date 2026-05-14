//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1031/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1031<F: Float>(t14757: F, t2376: F, t2409: F, t1112: F, t331: F, t2306: F, t3074: F, t833: F, t4157: F, t4414: F, t9716: F, t3959: F, t13989: F, t13999: F, t14002: F, t14114: F, t14742: F, t14745: F, t14749: F, t14752: F, t14755: F, t2408: F, t3066: F, t335: F) -> (F, F, F, F, F, F) {
    let t14759 = t2409 * t2376 * t14757;
    let t14765 = t1112 * t331;
    let t14766 = t2306 * t14765;
    let t14767 = t3074 * t14766;
    let t14768 = t14767 * t833;
    let t14770 = t4414 * t4157;
    let t14772 = t2409 * t9716;
    let t14773 = t3959 * t14772;
    let t14775 = -t335 * t14742 / 96.0 + 7.0 / 144.0 * t14745 + t3066 * t14749 / 48.0 + 7.0 / 288.0 * t14752 + t14755 / 1536.0 + t13989 + t2408 * t14759 / 48.0 - 7.0 / 144.0 * t13999 + 7.0 / 144.0 * t14002 + 7.0 / 1152.0 * t14114 + t14768 / 96.0 - 7.0 / 144.0 * t14770 - t14773 / 48.0;
    (t14759, t14765, t14766, t14767, t14772, t14775)
}
