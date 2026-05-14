//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1429/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1429<F: Float>(t10004: F, t112523: F, t113038: F, t113042: F, t116738: F, t116836: F, t118132: F, t118150: F, t118174: F, t121586: F, t121589: F, t121592: F, t121611: F, t121615: F, t122681: F, t2807: F, t34473: F, t9740: F) -> (F,) {
    let t122872 = -0.41270617283950617283e-2 * t116738 - t118132 - 0.92592592592592592593e-2 * t118150 + 0.27777777777777777778e-1 * t34473 * t10004 * t2807 - 0.11574074074074074074e-2 * t113038 + 0.92858888888888888886e-2 * t121586 + 0.11574074074074074074e-2 * t113042 + 0.23214722222222222221e-2 * t121589 + t118174 + 0.77382407407407407407e-3 * t121592 - 0.25794135802469135802e-3 * t112523 + 0.11349419753086419753e-1 * t121611 - 0.10446625e-1 * t121615 + 0.20635308641975308642e-2 * t116836 + 0.20833333333333333334e-1 * t9740 * t122681;
    (t122872,)
}
