//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 999/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk999<F: Float>(t10772: F, t237: F, t10779: F, t10782: F, t10785: F, t10870: F, t10894: F, t10896: F, t10898: F, t10900: F, t10903: F, t10921: F, t10930: F) -> (F, F) {
    let t10977 = F::cast_from(0.19751673498613801407e-1_f64) * t237 * t10772;
    let t10978 = t10894 - t10782 + t10785 - t10779 + t10977 + t10896 + t10898 + t10900 - t10903 + t10921 + t10870 + t10930;
    (t10977, t10978)
}
