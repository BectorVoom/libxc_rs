//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 800/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk800<F: Float>(t5543: F, t7444: F, t587: F, t1416: F, t2570: F, t1809: F, t1620: F, t1022: F, t1642: F, t1413: F, t2677: F, t7324: F, t7416: F, t7417: F, t7418: F, t7419: F, t7420: F, t7422: F, t7423: F, t7424: F, t7427: F, t7431: F, t7434: F, t7438: F, t7442: F) -> (F, F, F, F) {
    let t7445 = t5543 * t7444;
    let t7447 = 4.0 / 27.0 * t587 * t7445;
    let t7448 = t2570 * t1416;
    let t7449 = t1809 * t7448;
    let t7451 = 8.0 / 45.0 * t1620 * t7449;
    let t7452 = t1022 * t1642;
    let t7453 = t7452 * t1413;
    let t7454 = t2677 * t7453;
    let t7456 = 8.0 / 27.0 * t1620 * t7454;
    let t7457 = -t7324 - t7416 - t7417 - t7418 + t7419 + t7420 - t7422 - t7423 - t7424 + t7427 + t7431 + t7434 + t7438 - t7442 - t7447 + t7451 + t7456;
    (t7447, t7451, t7456, t7457)
}
