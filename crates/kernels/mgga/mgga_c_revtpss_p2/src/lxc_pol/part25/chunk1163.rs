//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1163/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1163<F: Float>(t265: F, t393: F, t1100: F, t1102: F, t198: F, t25705: F, t25709: F, t25713: F, t25743: F, t3329: F, t3333: F, t336: F, t5023: F, t7181: F) -> F {
    let t394 = t265 < t393;
    let t25744 = piecewise3::<F>(t394, t1102 * t198 * t25705 * t336 - F::new(2.0) * t1100 * t25709 * t5023 + F::new(2.0) * t25713 * t3333 * t5023 - t3329 * t5023 * t7181, t25743);
    t25744
}
