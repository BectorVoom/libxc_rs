//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1107/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1107<F: Float>(t2887: F, t2890: F, t487: F, t2003: F, t2888: F, t178: F, t17933: F, t17930: F, t18000: F, t18009: F, t2064: F, t2899: F, t2902: F) -> (F, F, F, F, F, F, F) {
    let t21359 = t2887 * t487 * t2890;
    let t21360 = t21359 / F::new(72.0);
    let t21395 = t2888 * t2003;
    let t21454 = t17933 * t178;
    let t21455 = t17930 * t21454;
    let t21462 = t18000 * t21454;
    let t21468 = t18009 * t21454;
    let t21499 = t2899 * t2064 * t2902;
    (t21360, t21395, t21454, t21455, t21462, t21468, t21499)
}
