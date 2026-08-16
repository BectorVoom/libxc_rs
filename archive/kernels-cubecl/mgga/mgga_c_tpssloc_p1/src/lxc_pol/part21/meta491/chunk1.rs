//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2099/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2099<F: Float>(t13087: F, t13182: F, t13190: F, t13202: F, t13208: F, t13234: F, t13237: F, t13262: F, t16836: F, t16841: F, t16845: F, t16848: F, t16853: F, t16859: F, t2618: F, t4172: F, t4178: F, t4184: F, t4257: F, t5587: F, t5614: F, t5619: F, t817: F, t843: F, t9602: F, t9672: F, t9967: F) -> F {
    let t16869 = -F::cast_from(35.0_f64) / F::cast_from(108.0_f64) * t13087 - F::cast_from(119.0_f64) / F::cast_from(3456.0_f64) * t9602 - F::cast_from(119.0_f64) / F::cast_from(6912.0_f64) * t13182 - t13190 + t13202 - t13208 + t16836 * t4184 / F::cast_from(768.0_f64) - t13262 * t16841 / F::cast_from(512.0_f64) + t4178 * t16845 / F::cast_from(512.0_f64) - F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t16848 - F::cast_from(119.0_f64) / F::cast_from(13824.0_f64) * t9672 - F::cast_from(5.0_f64) / F::cast_from(128.0_f64) * t843 * t16853 - t2618 * t5614 / F::cast_from(3072.0_f64) - t817 * t16859 / F::cast_from(3072.0_f64) - t2618 * t5619 / F::cast_from(3072.0_f64) + F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t4172 * t4257 + F::cast_from(119.0_f64) / F::cast_from(6912.0_f64) * t13234 - t13237 + t9967 * t5587 / F::cast_from(1536.0_f64);
    t16869
}
