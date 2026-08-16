//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1571/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1571<F: Float>(t486: F, t6224: F, t11721: F, t1215: F, t4582: F, t4978: F, t1222: F, t6170: F, t6158: F, t6165: F, t11644: F, t11649: F, t11719: F, t11728: F, t15446: F, t15448: F, t15450: F, t15452: F, t15503: F, t15507: F, t18297: F, t488: F, t4974: F, t4980: F, t4984: F, t5005: F) -> (F, F) {
    let t18300 = t486 * t6224;
    let t18301 = t11721 * t1215;
    let t18302 = t18300 * t18301;
    let t18303 = t4582 * t18302;
    let t18306 = t18300 * t4978;
    let t18307 = t4582 * t18306;
    let t18310 = t6170 * t1222;
    let t18312 = t6158 * t1222;
    let t18314 = t6165 * t1222;
    let t18316 = -t11644 / F::cast_from(13824.0_f64) + t11649 - t15503 * t4980 / F::cast_from(144.0_f64) + t15507 * t4984 / F::cast_from(288.0_f64) - t5005 * t4974 / F::cast_from(1152.0_f64) - t18297 * t488 / F::cast_from(288.0_f64) + t11719 * t18303 / F::cast_from(512.0_f64) - t11728 * t18307 / F::cast_from(512.0_f64) + t15446 - t15448 - t15450 + t15452 + t18310 / F::cast_from(4608.0_f64) - t18312 / F::cast_from(432.0_f64) + F::cast_from(19.0_f64) / F::cast_from(2592.0_f64) * t18314;
    (t18300, t18316)
}
