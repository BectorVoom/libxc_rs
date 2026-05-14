//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1070/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1070<F: Float>(t1579: F, t21499: F, t1588: F, t6174: F, t4401: F, t5670: F, t3973: F, t6501: F, t1580: F, t6481: F, t21480: F, t21484: F, t21489: F, t21494: F, t4381: F, t4397: F, t6477: F, t6502: F, t6507: F) -> (F, F, F) {
    let t21500 = t1579 * t21499;
    let t21501 = t6174 * t1588;
    let t21502 = t5670 * t4401;
    let t21503 = t21501 * t21502;
    let t21510 = t3973 * t6501;
    let t21511 = t1580 * t21510;
    let t21513 = t3973 * t6481;
    let t21515 = 0.59969295720591057378e-2 * t1580 * t21513;
    let t21516 = -0.17990788716177317213e-1 * t4397 * t6502 - 0.35981577432354634426e-1 * t1580 * t21480 - 0.17990788716177317213e-1 * t1580 * t21484 + 0.47975436576472845903e-1 * t1580 * t21489 - 0.89953943580886586067e-2 * t1580 * t21494 + 0.10794473229706390328e0 * t4397 * t6507 - 0.23987718288236422952e-1 * t21500 * t21503 - 0.63967248768630461204e-1 * t4381 * t6477 + 0.47975436576472845902e-1 * t4381 * t6502 - 0.59969295720591057378e-2 * t21511 - t21515;
    (t21500, t21502, t21516)
}
