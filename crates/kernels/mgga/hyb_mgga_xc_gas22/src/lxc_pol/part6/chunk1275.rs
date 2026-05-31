//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1275/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1275<F: Float>(t10013: F, t10022: F, t10043: F, t1211: F, t1947: F, t1955: F, t1959: F, t1976: F, t19975: F, t19990: F, t23488: F, t27474: F, t27530: F, t27571: F, t3072: F, t3876: F, t3881: F, t6088: F, t6096: F, t616: F, t72: F, t8061: F, t8074: F, t8077: F, t8102: F, t8103: F, t9999: F) -> F {
    let t27575 = -t8102 * t23488 - t10013 * t6088 / F::cast_from(4.0_f64) - t19990 * t3881 * t8103 / F::cast_from(8.0_f64) - F::cast_from(6.0_f64) * t6096 * t3881 * t1947 + F::cast_from(4.0_f64) * t1959 * t1211 * t8061 - t8074 * t10022 / F::cast_from(2.0_f64) - t3072 * t27474 - t8077 * t10022 / F::cast_from(4.0_f64) + F::cast_from(4.0_f64) * t1959 * t9999 * t616 + F::cast_from(2.0_f64) * t1959 * t3876 * t1947 - F::cast_from(24.0_f64) * t10043 * t8103 + F::cast_from(24.0_f64) * t19975 * t3881 * t1955 + F::cast_from(7.0_f64) / F::cast_from(2.0_f64) * t1976 * t10022 - F::cast_from(6.0_f64) * t6096 * t3876 * t1955 + F::cast_from(2.0_f64) * t72 * (t27530 + t27571);
    t27575
}
