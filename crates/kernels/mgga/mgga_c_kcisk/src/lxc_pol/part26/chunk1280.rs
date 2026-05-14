//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1280/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1280<F: Float>(t13955: F, t9815: F, t20160: F, t33388: F, t9426: F, t32102: F, t113581: F, t9446: F, t32105: F, t9801: F, t3805: F, t9824: F, t33515: F, t9442: F, t3959: F, t399: F) -> (F, F, F, F, F, F, F, F, F) {
    let t114302 = t13955 * t9815;
    let t114304 = t20160 * t33388;
    let t114305 = t9426 * t114304;
    let t114315 = 0.15520416666666666667e-2 * t32102 * t114304;
    let t114351 = t9446 * t113581;
    let t114361 = t9801 * t32105;
    let t114368 = t3805 * t9824;
    let t114377 = 0.69444444444444444446e-2 * t33515 * t9442;
    let t114378 = t399 * t3959;
    (t114302, t114304, t114305, t114315, t114351, t114361, t114368, t114377, t114378)
}
