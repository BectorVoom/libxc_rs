//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1214/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1214<F: Float>(t25583: F, t92: F, t1286: F, t1637: F, t6460: F, t10951: F, t91493: F, t25621: F, t376: F, t25584: F, t5498: F, t22914: F, t25602: F, t101678: F, t1755: F, t22495: F, t22900: F, t23133: F, t28: F, t5504: F, t5507: F, t5508: F, t6414: F, t6461: F, t93927: F, t93946: F, t984: F) -> (F, F) {
    let t102033 = t25583 * t92;
    let t102037 = t1286 * t1637 * t6460;
    let t102045 = t91493 * t10951;
    let t102049 = t1286 * t376 * t25621 / 9.0;
    let t102051 = t25584 * t5498 / 9.0;
    let t102053 = 2.0 / 27.0 * t22914 * t25602;
    let t102054 = t93927 / 81.0 - t1286 * t28 * t5507 * t984 * t1755 / 3.0 - 2.0 / 3.0 * t1286 * t28 * t101678 * t5508 - t102033 * t5504 / 9.0 - t93946 + 2.0 / 27.0 * t102037 - 2.0 / 3.0 * t6414 * t22495 + t6414 * t22900 / 6.0 + t23133 * t6461 / 6.0 - 12.0 * t102045 - t102049 - t102051 - t102053;
    (t102045, t102054)
}
