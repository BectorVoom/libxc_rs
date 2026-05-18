//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 616/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk616<F: Float>(t1729: F, t776: F, t2306: F, t684: F, t2310: F, t1738: F, t872: F, t1529: F, t822: F, t1982: F, t515: F, t1960: F, t568: F) -> (F, F, F, F, F, F, F) {
    let t4449 = t1729 * t776;
    let t4454 = F::new(0.039914113367515366) * t684 * t2306;
    let t4455 = t684 * t2310;
    let t4457 = t1738 * t872;
    let t4465 = t822 * t1529;
    let t4468 = F::new(8.0) / F::new(45.0) * t1982 * t515;
    let t4470 = F::new(8.0) / F::new(45.0) * t1960 * t568;
    (t4449, t4454, t4455, t4457, t4465, t4468, t4470)
}
