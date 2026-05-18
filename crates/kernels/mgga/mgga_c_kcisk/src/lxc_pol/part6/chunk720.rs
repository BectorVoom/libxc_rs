//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 720/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk720<F: Float>(t12809: F, t190: F, t214: F, t12694: F, t12701: F, t12703: F, t12706: F, t12708: F, t12710: F, t12714: F, t12717: F, t12771: F, t12774: F, t12776: F, t12779: F, t12782: F) -> (F, F) {
    let t12810 = t12809 * t190;
    let t12811 = t12810 * t214;
    let t12813 = F::new(3.0) / F::new(4.0) * t12694 + F::new(3.0) / F::new(32.0) * t12701 - F::new(9.0) / F::new(4.0) * t12703 + F::new(3.0) / F::new(64.0) * t12706 - F::new(3.0) / F::new(8.0) * t12708 - F::new(3.0) / F::new(8.0) * t12710 - F::new(3.0) / F::new(4.0) * t12714 + F::new(3.0) / F::new(8.0) * t12717 + t12771 / F::new(64.0) - F::new(3.0) / F::new(16.0) * t12774 + F::new(3.0) / F::new(64.0) * t12776 - t12779 / F::new(8.0) + F::new(3.0) / F::new(4.0) * t12782 - t12811 / F::new(64.0);
    (t12811, t12813)
}
