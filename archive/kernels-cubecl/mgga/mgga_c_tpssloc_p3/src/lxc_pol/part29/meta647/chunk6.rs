//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2148/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2148<F: Float>(t25038: F, t25248: F, t776: F, t87130: F, t22986: F, t6646: F, t829: F, t87111: F, t82039: F, t25273: F, t6579: F, t244: F, t268: F, t6559: F) -> (F, F, F, F, F) {
    let t87699 = t25038 * t25248 * t87130 * t776;
    let t87705 = t22986 * t6646 * t87111 * t829;
    let t87708 = F::cast_from(0.10417915756705434098e0_f64) * t82039;
    let t87709 = t6579 * t25273;
    let t87710 = F::cast_from(0.38381794893125283518e-1_f64) * t87709;
    let t87712 = t6559 * t244 * t268;
    (t87699, t87705, t87708, t87710, t87712)
}
