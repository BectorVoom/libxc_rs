//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1091/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1091<F: Float>(t411: F, t7970: F, t5651: F, t14657: F, t19516: F, t19518: F, t8902: F, t14783: F, t19523: F, t19526: F, t19533: F, t19540: F, t19544: F, t8899: F) -> (F, F, F, F, F, F) {
    let t20293 = t7970 * t411;
    let t20294 = t5651 * t20293;
    let t20301 = F::cast_from(52.61445_f64) * t14657 * t20293;
    let t20302 = F::cast_from(3.8973666666666666_f64) * t19516;
    let t20303 = F::cast_from(1.9486833333333333_f64) * t19518;
    let t20305 = F::cast_from(1.9486833333333333_f64) * t8902;
    let t20311 = -F::cast_from(88.1424_f64) * t14783 * t20293 - t20301 + t20302 - t20303 - F::cast_from(0.97936_f64) * t8899 + t20305 + F::cast_from(5.87616_f64) * t19523 - F::cast_from(1.46904_f64) * t19526 + F::cast_from(2.0_f64) * t19533 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t19540 + F::cast_from(11.75232_f64) * t19544;
    (t20294, t20301, t20302, t20303, t20305, t20311)
}
