//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2677/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2677<F: Float>(t16398: F, t20470: F, t12419: F, t1307: F, t16242: F, t20448: F, t20500: F, t210: F, t3733: F, t3803: F, t54132: F, t54151: F, t56837: F, t56853: F, t56883: F, t56885: F, t56888: F, t56906: F, t56909: F, t56919: F, t56921: F) -> F {
    let t74618 = t16398 * t20470;
    let t74632 = -F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t3803 * t12419 * t16242 * t20448 + F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t56837 - F::cast_from(7.0_f64) / F::cast_from(192.0_f64) * t56853 + F::cast_from(7.0_f64) / F::cast_from(192.0_f64) * t74618 + t54132 + F::cast_from(7.0_f64) / F::cast_from(256.0_f64) * t56883 - F::cast_from(7.0_f64) / F::cast_from(256.0_f64) * t56885 - F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t56888 - F::cast_from(7.0_f64) / F::cast_from(192.0_f64) * t56906 + t3733 * t210 * t20500 * t1307 / F::cast_from(16.0_f64) + F::cast_from(595.0_f64) / F::cast_from(3456.0_f64) * t54151 - F::cast_from(35.0_f64) / F::cast_from(192.0_f64) * t56909 + F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t56919 + F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t56921;
    t74632
}
