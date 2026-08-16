//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1273/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1273<F: Float>(t43: F, t27405: F, t81: F, t9999: F, t3876: F, t6127: F, t10013: F, t10022: F, t10043: F, t10046: F, t10057: F, t10063: F, t1954: F, t1967: F, t19952: F, t19960: F, t23488: F, t3086: F, t3087: F, t3093: F, t3099: F, t3881: F, t3882: F, t3898: F, t6088: F, t8103: F, t8130: F) -> (F, F, F) {
    let t45 = F::cast_from(0.135e1_f64) < t43;
    let t27443 = piecewise3::<F>(t45, F::cast_from(0.0_f64), t27405);
    let t27474 = t81 * t9999;
    let t27499 = t6127 * t3876;
    let t27530 = -F::cast_from(75.0_f64) / F::cast_from(2.0_f64) * t3898 * t8103 + F::cast_from(15.0_f64) / F::cast_from(2.0_f64) * t1954 * t3876 * t8103 + t27499 * t8103 / F::cast_from(8.0_f64) + t3093 * t23488 / F::cast_from(2.0_f64) + t10057 * t6088 / F::cast_from(8.0_f64) + t19960 * t3881 * t8103 / F::cast_from(16.0_f64) - F::cast_from(2.0_f64) * t10063 * t23488 - t8130 * t10022 - F::cast_from(2.0_f64) * t3099 * t27474 + F::cast_from(15.0_f64) / F::cast_from(2.0_f64) * t3882 * t6088 + F::cast_from(85.0_f64) / F::cast_from(4.0_f64) * t10013 * t8103 - F::cast_from(4.0_f64) * t3086 * t23488 - F::cast_from(5.0_f64) / F::cast_from(2.0_f64) * t10043 * t6088 - F::cast_from(19.0_f64) / F::cast_from(8.0_f64) * t19952 * t3881 * t8103 - F::cast_from(4.0_f64) * t1967 * t9999 * t3087 - F::cast_from(2.0_f64) * t10046 * t6088;
    (t27443, t27474, t27530)
}
