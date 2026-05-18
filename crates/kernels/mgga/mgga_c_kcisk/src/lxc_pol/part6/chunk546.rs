//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 546/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk546<F: Float>(t222: F, t224: F, t3277: F, t7706: F, t7710: F, t2063: F, zeta_threshold: F) -> (F, F) {
    let t223 = t222 <= zeta_threshold;
    let t7714 = piecewise3::<f64>(t223, F::new(0.0), F::new(4.0) / F::new(9.0) * t3277 * t7706 + F::new(4.0) / F::new(3.0) * t224 * t7710);
    let t7715 = t2063 * t2063;
    (t7714, t7715)
}
