//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1994/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1994<F: Float>(t644: F, t6977: F, t1927: F, t2315: F, t2247: F, t2259: F, t843: F, t1962: F, t41154: F, t2411: F, t25435: F, t605: F) -> (F, F, F, F, F, F, F) {
    let t92576 = t6977 * t644;
    let t92584 = t1927 * t2315;
    let t92588 = t2247 * t2259;
    let t92612 = F::cast_from(1232.0_f64) / F::cast_from(27.0_f64) * t843;
    let t92742 = t1962 * t41154;
    let t92775 = t25435 * t2411;
    let t92790 = t2411 * t605;
    (t92576, t92584, t92588, t92612, t92742, t92775, t92790)
}
