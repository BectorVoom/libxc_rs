//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1698/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1698<F: Float>(t23714: F, t4724: F, t981: F, t4711: F, t78429: F, t23446: F, t4719: F, t23453: F, t19049: F, t6219: F, t88510: F, t88562: F, t88564: F, t88567: F, t88600: F, t88602: F, t88607: F) -> (F, F, F, F, F, F) {
    let t88986 = F::cast_from(0.46785788981077169656e1_f64) * t981 * t4724 * t23714;
    let t88989 = F::cast_from(0.69263436422725855036e2_f64) * t981 * t78429 * t4711;
    let t88991 = F::cast_from(0.14035736694323150897e2_f64) * t4719 * t23446;
    let t88993 = F::cast_from(0.4155806185363551302e3_f64) * t4719 * t23453;
    let t88995 = F::cast_from(0.70178683471615754484e1_f64) * t19049 * t6219;
    let t88996 = t88600 - t88602 + t88510 - t88607 - t88562 + t88564 - t88567 + t88986 - t88989 + t88991 + t88993 + t88995;
    (t88986, t88989, t88991, t88993, t88995, t88996)
}
