//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1035/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1035<F: Float>(t1205: F, t3306: F, t2409: F, t3067: F, t4216: F, t8734: F, t1105: F, t4110: F, t2376: F, t14185: F, t3060: F, t9283: F, t14655: F, t4218: F, t9270: F, t14295: F, t14302: F, t14305: F, t14634: F, t14640: F, t14649: F, t14658: F, t2408: F, t3066: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14943 = t1205 * t3306;
    let t14945 = t2409 * t3067 * t14943;
    let t14949 = t2409 * t8734 * t4216;
    let t14952 = t4110 * t1105;
    let t14954 = t2409 * t2376 * t14952;
    let t14958 = t14185 * t3060;
    let t14959 = t9283 * t14958;
    let t14962 = 7.0 / 576.0 * t14655;
    let t14964 = t9270 * t4218;
    let t14967 = t14634 / 384.0 + 5.0 / 384.0 * t14640 + t3066 * t14945 / 48.0 + t3066 * t14949 / 48.0 + t2408 * t14954 / 48.0 - t14649 / 48.0 - t2408 * t14959 / 24.0 + t14295 + t14962 - t14302 - t14658 / 48.0 - 7.0 / 144.0 * t14964 - 7.0 / 144.0 * t14305;
    (t14943, t14945, t14949, t14952, t14954, t14958, t14959, t14964, t14967)
}
