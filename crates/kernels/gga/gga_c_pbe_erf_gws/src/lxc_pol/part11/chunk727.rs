//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 727/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk727<F: Float>(t12702: F, t186: F, t185: F, t2741: F, t3564: F, t12344: F, t220: F, t616: F, t10743: F, t10938: F, t950: F, t1827: F, t587: F, t3411: F, t7130: F, t10424: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12703 = t186 * t12702;
    let t12705 = 2.0 / 15.0 * t185 * t12703;
    let t12707 = 4.0 / 5.0 * t2741 * t3564;
    let t12709 = -3.0 * t12344;
    let t12710 = t220 * t12709;
    let t12711 = t186 * t12710;
    let t12713 = 4.0 / 15.0 * t616 * t12711;
    let t12715 = 4.0 / 5.0 * t10743 * t3564;
    let t12716 = t10938 * t950;
    let t12717 = t1827 * t12716;
    let t12719 = 4.0 / 15.0 * t587 * t12717;
    let t12721 = 16.0 / 15.0 * t7130 * t3411;
    let t12722 = t10424 * t950;
    (t12703, t12705, t12707, t12709, t12710, t12711, t12713, t12715, t12716, t12717, t12719, t12721, t12722)
}
