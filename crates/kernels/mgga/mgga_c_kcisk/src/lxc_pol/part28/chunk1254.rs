//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1254/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1254<F: Float>(t33220: F, t9212: F, t33219: F, t10004: F, t9990: F, t10005: F, t2804: F, t2807: F, t33177: F, t33196: F, t33278: F, t34278: F, t34280: F, t34573: F, t34600: F, t35222: F, t35225: F, t35234: F, t35250: F, t35254: F, t35402: F, t35416: F, t35454: F, t35476: F, t9740: F, t9995: F) -> (F, F, F, F) {
    let t35505 = t33220 * t9212;
    let t35506 = t33219 * t35505;
    let t35511 = t9990 * t10004;
    let t35519 = -0.92858888888888888886e-2 * t35222 + 0.52083333333333333333e-2 * t2804 * t35476 - 0.10722222222222222222e-1 * t34573 * t9995 + 0.17411041666666666666e-2 * t35225 - 0.27777777777777777778e-1 * t10005 * t9995 - 0.10416666666666666667e-1 * t9740 * t35402 + t33278 - 0.17411041666666666666e-2 * t35234 + 0.13402777777777777778e-2 * t33196 * t35454 + 0.34722222222222222222e-2 * t9740 * t35506 - 0.116403125e-2 * t33177 * t35416 + 0.27777777777777777778e-1 * t35511 * t2807 + 0.17024129629629629629e-1 * t35250 - 0.23214722222222222222e-2 * t35254 - 0.34722222222222222222e-2 * t34600 + 0.23214722222222222222e-2 * t34278 - 0.61905925925925925925e-2 * t34280;
    (t35505, t35506, t35511, t35519)
}
