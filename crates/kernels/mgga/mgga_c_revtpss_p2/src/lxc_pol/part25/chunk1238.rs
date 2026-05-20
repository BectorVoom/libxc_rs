//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1238/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1238<F: Float>(t7030: F, t9784: F, t10788: F, t27261: F, t2482: F, t25260: F, t27: F, t10852: F, t25266: F, t2756: F, t10836: F, t25227: F, t2661: F) -> (F, F, F, F, F) {
    let t93020 = t9784 * t7030;
    let t93021 = F::cast_from(0.14450132032386466905e-2_f64) * t93020;
    let t93022 = t27261 * t10788;
    let t93025 = t2482 * t25260 * t27;
    let t93026 = t93025 * t10852;
    let t93028 = t25266 * t2756;
    let t93031 = t2661 * t25227 * t10836;
    (t93021, t93022, t93026, t93028, t93031)
}
