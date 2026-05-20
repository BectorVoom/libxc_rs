//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1869/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1869<F: Float>(t94701: F, t96204: F, t26359: F, t9303: F, t13790: F, t4102: F, t685: F, t72: F, t1444: F, t5740: F, t675: F, t14109: F, t25900: F) -> (F, F, F, F, F) {
    let t96584 = F::cast_from(0.51727911450665971904e-3_f64) * t94701 * t96204;
    let t96591 = F::cast_from(0.26019841438354088051e-2_f64) * t9303 * t26359;
    let t97680 = t13790 * t72 * t685 * t4102;
    let t97685 = t5740 * t685 * t675 * t1444;
    let t97688 = t14109 * t25900;
    (t96584, t96591, t97680, t97685, t97688)
}
