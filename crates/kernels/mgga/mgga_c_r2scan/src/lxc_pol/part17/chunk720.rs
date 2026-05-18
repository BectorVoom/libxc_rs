//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 720/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk720<F: Float>(t5511: F, t5562: F, t5610: F, t5647: F, t5702: F, t5744: F, t5797: F, t5841: F, t61: F, t41: F, t1419: F, t661: F) -> (F, F) {
    let t5844 = t5511 + t5562 + t5610 + t5647 + t5702 + t5744 + t5797 + t5841;
    let t5845 = t61 * t5844;
    let t5846 = t41 * t5845;
    let t5847 = t1419 * t661;
    (t5846, t5847)
}
