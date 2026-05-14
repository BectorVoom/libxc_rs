//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 962/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk962<F: Float>(t411: F, t7970: F, t5651: F, t14657: F, t19516: F, t19518: F, t8902: F, t14783: F, t19523: F, t19526: F, t19533: F, t19540: F, t19544: F, t8899: F, t9003: F, t9017: F) -> (F, F, F, F, F, F, F, F) {
    let t20293 = t7970 * t411;
    let t20294 = t5651 * t20293;
    let t20301 = 52.61445 * t14657 * t20293;
    let t20302 = 3.8973666666666666 * t19516;
    let t20303 = 1.9486833333333333 * t19518;
    let t20305 = 1.9486833333333333 * t8902;
    let t20311 = -88.1424 * t14783 * t20293 - t20301 + t20302 - t20303 - 0.97936 * t8899 + t20305 + 5.87616 * t19523 - 1.46904 * t19526 + 2.0 * t19533 - 2.0 / 3.0 * t19540 + 11.75232 * t19544;
    let t20318 = 1.5156425925925925 * t9003;
    let t20319 = 1.2991222222222223 * t9017;
    (t20294, t20301, t20302, t20303, t20305, t20311, t20318, t20319)
}
