//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1147/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1147<F: Float>(t33382: F, t33420: F, t33454: F, t33487: F, t33518: F, t33553: F, t33580: F, t33613: F, t504: F, t1458: F, t9827: F, t1520: F, t2282: F, t32226: F, t32229: F, t6244: F) -> (F, F, F, F, F, F) {
    let t33616 = t33382 + t33420 + t33454 + t33487 + t33518 + t33553 + t33580 + t33613;
    let t33617 = t33616 * t504;
    let t33618 = t9827 * t1458;
    let t33619 = t33618 * t1520;
    let t33620 = t32226 * t2282;
    let t33622 = 2.0 * t32229 * t6244;
    (t33616, t33617, t33618, t33619, t33620, t33622)
}
