//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1750/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1750<F: Float>(t17092: F, t24212: F, t16840: F, t24215: F, t6534: F, t1196: F, t3520: F, t3523: F, t6518: F) -> (F, F, F, F, F) {
    let t90349 = F::new(24.0) * t17092 * t24212;
    let t90351 = F::cast_from(0.1929837539843104208e3_f64) * t16840 * t24215;
    let t90352 = t6534 * t6534;
    let t90356 = F::cast_from(0.51947577317044391277e2_f64) * t1196 * t3520 * t90352 * t3523;
    let t90357 = t6518 * t6518;
    (t90349, t90351, t90352, t90356, t90357)
}
