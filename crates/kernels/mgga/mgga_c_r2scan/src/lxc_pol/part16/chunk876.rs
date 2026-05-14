//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 876/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk876<F: Float>(t4176: F, t986: F, t3270: F, t3269: F, t1108: F, t2449: F, t1065: F, t983: F, t11002: F, t10663: F, t3579: F, t2526: F, t797: F, t3262: F, t3263: F, t2333: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11539 = t4176 * t986;
    let t11540 = t3270 * t11539;
    let t11541 = t3269 * t11540;
    let t11542 = t11541 / 4.0;
    let t11543 = t2449 * t1108;
    let t11544 = t1065 * t983;
    let t11545 = t11002 * t11544;
    let t11546 = t3269 * t11545;
    let t11547 = 5.0 / 16.0 * t11546;
    let t11548 = t3579 * t10663;
    let t11549 = t11548 / 4.0;
    let t11550 = t797 * t2526;
    let t11552 = t3262 * t3263 * t11550;
    let t11553 = 3.0 / 4.0 * t11552;
    let t11554 = t2333 * t983;
    (t11540, t11541, t11542, t11543, t11545, t11546, t11547, t11548, t11549, t11550, t11552, t11553, t11554)
}
