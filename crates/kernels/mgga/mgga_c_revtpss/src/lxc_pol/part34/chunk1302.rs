//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1302/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1302<F: Float>(t265: F, t393: F, t100806: F, t107741: F, t1102: F, t113678: F, t113728: F, t113774: F, t113819: F, t113867: F, t113912: F, t113961: F, t114009: F, t114089: F, t1699: F, t198: F, t23571: F, t24186: F, t25713: F, t27712: F, t336: F, t5023: F, t6396: F, t6400: F, t7181: F, t94149: F) -> F {
    let t394 = t265 < t393;
    let t114090 = piecewise3::<f64>(t394, t198 * t336 * (t113678 + t113728 + t113774 + t113819 + t113867 + t113912 + t113961 + t114009) * t1102 - F::new(3.0) * t5023 * t107741 * t1699 + F::new(6.0) * t5023 * t100806 * t6400 - F::new(3.0) * t5023 * t27712 * t6396 - F::new(6.0) * t5023 * t94149 * t23571 + F::new(6.0) * t5023 * t25713 * t1699 * t6396 - t5023 * t7181 * t24186, t114089);
    t114090
}
