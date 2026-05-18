//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 778/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk778<F: Float>(t12647: F, t639: F, t1017: F, t11110: F, t1885: F, t587: F, t1046: F, t3493: F, t10686: F, t3535: F, t7130: F, t10908: F, t995: F) -> (F, F, F, F, F, F, F, F) {
    let t12649 = F::new(4.0) / F::new(5.0) * t639 * t12647;
    let t12650 = t11110 * t1017;
    let t12651 = t1885 * t12650;
    let t12653 = F::new(4.0) / F::new(5.0) * t587 * t12651;
    let t12655 = F::new(4.0) / F::new(5.0) * t3493 * t1046;
    let t12656 = F::new(16.0) / F::new(45.0) * t10686;
    let t12658 = F::new(8.0) / F::new(5.0) * t7130 * t3535;
    let t12659 = t10908 * t995;
    (t12649, t12650, t12651, t12653, t12655, t12656, t12658, t12659)
}
