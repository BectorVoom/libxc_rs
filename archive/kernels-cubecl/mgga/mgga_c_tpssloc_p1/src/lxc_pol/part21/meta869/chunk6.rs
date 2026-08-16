//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3188/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3188<F: Float>(t15590: F, t5018: F, t15507: F, t15548: F, t1218: F, t15438: F, t15503: F, t15531: F, t15535: F, t15555: F, t15622: F, t15627: F, t18307: F, t18346: F, t3490: F, t44858: F, t44953: F, t4980: F, t52810: F, t52836: F, t52952: F, t52973: F, t52975: F, t52987: F, t53336: F) -> F {
    let t66159 = t15590 * t5018;
    let t66165 = t15507 * t15548;
    let t66185 = -t66159 * t1218 / F::cast_from(144.0_f64) + t44953 / F::cast_from(10368.0_f64) - t52810 * t4980 / F::cast_from(72.0_f64) + t66165 / F::cast_from(216.0_f64) + t52952 / F::cast_from(3456.0_f64) - t44858 * t18307 / F::cast_from(256.0_f64) + F::cast_from(5.0_f64) / F::cast_from(1152.0_f64) * t3490 * t18346 - t52973 / F::cast_from(3456.0_f64) + t52975 / F::cast_from(324.0_f64) - t15438 * t15531 / F::cast_from(1536.0_f64) + t52836 * t15535 / F::cast_from(1536.0_f64) - t15503 * t15555 / F::cast_from(72.0_f64) - t15503 * t15622 / F::cast_from(144.0_f64) - t53336 * t15627 / F::cast_from(48.0_f64) + t52987 / F::cast_from(648.0_f64);
    t66185
}
