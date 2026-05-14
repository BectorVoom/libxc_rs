//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1064/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1064<F: Float>(t497: F, t6454: F, t108: F, t25846: F, t22914: F, t25574: F, t25583: F, t92: F, t1286: F, t1637: F, t6460: F, t25621: F, t376: F, t25584: F, t5498: F, t25602: F) -> (F, F, F, F, F, F, F, F) {
    let t101975 = t6454 * t497;
    let t101983 = t25846 * t108;
    let t102018 = t22914 * t25574 / 27.0;
    let t102033 = t25583 * t92;
    let t102037 = t1286 * t1637 * t6460;
    let t102049 = t1286 * t376 * t25621 / 9.0;
    let t102051 = t25584 * t5498 / 9.0;
    let t102053 = 2.0 / 27.0 * t22914 * t25602;
    (t101975, t101983, t102018, t102033, t102037, t102049, t102051, t102053)
}
