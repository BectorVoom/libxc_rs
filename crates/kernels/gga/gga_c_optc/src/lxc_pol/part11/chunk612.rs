//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 612/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk612<F: Float>(t5064: F, t914: F, t2549: F, t4768: F, t1000: F, t4772: F, t1435: F, t2354: F, t2569: F, t277: F, t3975: F, t4054: F, t4897: F, t4952: F, t4956: F, t4960: F, t5059: F, t95: F, t999: F) -> (F, F, F, F, F, F) {
    let t5065 = t914 * t5064;
    let t5068 = t2549 * t4768;
    let t5069 = t914 * t5068;
    let t5075 = t1000 * t4772;
    let t5076 = t914 * t5075;
    let t5079 = -t4952 - t4956 - t4960 - F::cast_from(0.25844881434903430496e-2_f64) * t95 * t277 * t5059 * t2569 - t4897 + t999 * t5065 / F::cast_from(6.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t999 * t5069 + t4054 * t1435 / F::cast_from(3.0_f64) - t2354 + t3975 / F::cast_from(9.0_f64) - t999 * t5076 / F::cast_from(3.0_f64);
    (t5065, t5068, t5069, t5075, t5076, t5079)
}
