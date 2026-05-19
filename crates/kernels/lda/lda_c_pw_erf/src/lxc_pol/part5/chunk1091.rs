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
    let t20301 = F::new(52.61445) * t14657 * t20293;
    let t20302 = F::cast_from(3.8973666666666666_f64) * t19516;
    let t20303 = F::cast_from(1.9486833333333333_f64) * t19518;
    let t20305 = F::cast_from(1.9486833333333333_f64) * t8902;
    let t20311 = -F::new(88.1424) * t14783 * t20293 - t20301 + t20302 - t20303 - F::new(0.97936) * t8899 + t20305 + F::new(5.87616) * t19523 - F::new(1.46904) * t19526 + F::new(2.0) * t19533 - F::new(2.0) / F::new(3.0) * t19540 + F::new(11.75232) * t19544;
    (t20294, t20301, t20302, t20303, t20305, t20311)
}
