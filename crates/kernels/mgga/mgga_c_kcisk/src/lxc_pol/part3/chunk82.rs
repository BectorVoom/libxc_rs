//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 82/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk82<F: Float>(t60: F, t67: F, t10: F, t260: F, t116: F) -> (F, F) {
    let t261 = t67 * t60;
    let t264 = F::new(10.0) / F::new(9.0) * t260 * t261 * t10;
    let t265 = t264 < -F::new(0.66725e-1);
    let t267 = piecewise3::<f64>(t265, F::new(0.0), F::new(0.66725e-1) + t264);
    let t268 = t267 * t116;
    (t261, t268)
}
