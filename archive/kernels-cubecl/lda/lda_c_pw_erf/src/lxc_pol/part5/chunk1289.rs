//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1289/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1289<F: Float>(t18523: F, t18551: F, t18575: F, t18584: F, t18593: F, t18596: F, t18599: F, t18615: F, t18630: F, t18642: F, t18655: F, t15152: F, t15153: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t23053 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t18523;
    let t23054 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t18551;
    let t23055 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t18575;
    let t23056 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t18584;
    let t23057 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t18593;
    let t23058 = F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t18596;
    let t23059 = F::cast_from(8.0_f64) / F::cast_from(81.0_f64) * t18599;
    let t23060 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t18615;
    let t23061 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t18630;
    let t23062 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t18642;
    let t23064 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t18655;
    let t23065 = -t23053 - t23054 + t23055 - t23056 + t23057 - t23058 - t23059 - t23060 - t23061 + t23062 + t15152 - F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t15153 - t23064;
    (t23053, t23054, t23055, t23056, t23057, t23058, t23059, t23060, t23061, t23062, t23064, t23065)
}
