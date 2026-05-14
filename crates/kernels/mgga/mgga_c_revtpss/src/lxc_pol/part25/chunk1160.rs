//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1160/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1160<F: Float>(t265: F, t393: F, t1100: F, t1102: F, t11105: F, t12190: F, t198: F, t25709: F, t25713: F, t3329: F, t3333: F, t336: F, t5023: F, t52188: F, t7181: F, t93458: F, t93514: F, t93852: F, t93907: F, t93958: F, t94021: F, t94075: F, t94131: F, t94138: F, t94142: F, t94149: F, t94213: F) -> (F,) {
    let t394 = t265 < t393;
    let t94214 = piecewise3(t394, t198 * t336 * (t93458 + t93514 + t93852 + t93907 + t93958 + t94021 + t94075 + t94131) * t1102 - 3.0 * t5023 * t94138 * t1100 + 6.0 * t5023 * t94142 * t3333 - 3.0 * t5023 * t25709 * t3329 - 6.0 * t5023 * t94149 * t11105 + 6.0 * t5023 * t25713 * t52188 - t5023 * t7181 * t12190, t94213);
    (t94214,)
}
