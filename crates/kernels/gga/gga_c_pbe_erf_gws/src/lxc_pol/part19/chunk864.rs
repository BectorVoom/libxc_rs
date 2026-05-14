//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 864/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk864<F: Float>(t3473: F, t661: F, t1815: F, t639: F, t10550: F, t1809: F, t10555: F, t2677: F, t3465: F, t617: F, t1620: F, t1022: F, t1044: F, t7216: F, t2576: F, t7527: F) -> (F, F, F, F, F, F) {
    let t10708 = t3473 * t661;
    let t10709 = t1815 * t10708;
    let t10711 = 4.0 / 45.0 * t639 * t10709;
    let t10712 = t1809 * t10550;
    let t10714 = 8.0 / 45.0 * t639 * t10712;
    let t10715 = t2677 * t10555;
    let t10717 = 4.0 / 27.0 * t639 * t10715;
    let t10718 = t3465 * t617;
    let t10719 = t2677 * t10718;
    let t10721 = 8.0 / 27.0 * t1620 * t10719;
    let t10722 = t1022 * t1044;
    let t10723 = t10722 * t661;
    let t10724 = t7216 * t10723;
    let t10726 = 16.0 / 15.0 * t1620 * t10724;
    let t10728 = 16.0 / 45.0 * t7527 * t2576;
    (t10711, t10714, t10717, t10721, t10726, t10728)
}
