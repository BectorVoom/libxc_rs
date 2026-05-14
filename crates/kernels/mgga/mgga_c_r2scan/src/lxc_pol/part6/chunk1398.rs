//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1398/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1398<F: Float>(t26475: F, t21340: F, t21344: F, t21345: F, t21349: F, t21350: F, t21354: F, t21357: F, t21361: F, t21365: F, t21370: F, t21371: F, t21375: F, t1726: F, t5364: F, t955: F) -> (F, F) {
    let t26476 = 0.4051561992e0 * t26475;
    let t26477 = t21340 - t21344 + 0.97592231702715658578e-1 * t21345 + t21349 - 180.0 * t21350 - t21354 + t21357 + t21361 + t21365 + t21370 - 0.50808839199999999998e-2 * t21371 + t26476 + t21375;
    let t26481 = t1726 * t955 * t5364;
    (t26477, t26481)
}
