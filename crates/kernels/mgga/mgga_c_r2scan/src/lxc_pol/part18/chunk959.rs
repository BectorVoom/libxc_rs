//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 959/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk959<F: Float>(t11539: F, t3270: F, t3269: F, t1108: F, t2449: F, t1065: F, t983: F, t11002: F, t10663: F, t3579: F, t2526: F, t797: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11540 = t3270 * t11539;
    let t11541 = t3269 * t11540;
    let t11542 = t11541 / F::new(4.0);
    let t11543 = t2449 * t1108;
    let t11544 = t1065 * t983;
    let t11545 = t11002 * t11544;
    let t11546 = t3269 * t11545;
    let t11547 = F::new(5.0) / F::new(16.0) * t11546;
    let t11548 = t3579 * t10663;
    let t11549 = t11548 / F::new(4.0);
    let t11550 = t797 * t2526;
    (t11540, t11541, t11542, t11543, t11545, t11546, t11547, t11548, t11549, t11550)
}
