//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1009/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1009<F: Float>(t10870: F, t10891: F, t10894: F, t10896: F, t10898: F, t10900: F, t10903: F, t10921: F, t10958: F, t10968: F, t10974: F, t10977: F, t1147: F, t1306: F, t9725: F) -> F {
    let t11117 = -F::new(3.0) * t1147 * t1306 * t9725 + t10870 - t10891 + t10894 + t10896 + t10898 + t10900 - t10903 + t10921 - t10958 - t10968 + t10974 + t10977;
    t11117
}
