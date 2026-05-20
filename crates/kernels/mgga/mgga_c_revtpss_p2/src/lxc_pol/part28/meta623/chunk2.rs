//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2209/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2209<F: Float>(t25512: F, t4820: F, t25515: F, t370: F, t16087: F, t4890: F, t93595: F, t16055: F, t27493: F, t15925: F, t25516: F, t1087: F, t93751: F) -> (F, F, F, F, F, F, F) {
    let t100006 = F::cast_from(0.57165357490759649296e-3_f64) * t25512 * t4820;
    let t100007 = t25515 * t370;
    let t100008 = t16087 * t100007;
    let t100019 = t93595 * t4890;
    let t100024 = F::cast_from(0.11433071498151929859e-2_f64) * t27493 * t16055;
    let t100025 = t15925 * t25516;
    let t100030 = t1087 * t93751;
    (t100006, t100007, t100008, t100019, t100024, t100025, t100030)
}
