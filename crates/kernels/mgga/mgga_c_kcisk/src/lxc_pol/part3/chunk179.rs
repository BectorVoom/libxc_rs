//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 179/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk179<F: Float>(t429: F, t430: F, t435: F, t436: F, t445: F, t446: F, t674: F, t686: F, t690: F, t696: F, t698: F) -> F {
    let t702 = -F::new(0.11955719325063177623e-1) * t674 + F::new(0.263475e-2) * t429 * t430 * t686 - F::new(0.4755e-3) * t435 * t436 * t690 + F::new(0.2589769453898153438e-4) * t696 - F::new(0.21605625e-5) * t445 * t446 * t698;
    t702
}
