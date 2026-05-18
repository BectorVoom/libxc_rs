//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1199/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1199<F: Float>(t4199: F, t4546: F, t4207: F, t4589: F, t515: F, t172: F, t184: F, t4645: F, t496: F, t10043: F, t10053: F, t10092: F, t10098: F, t10100: F, t10115: F, t13568: F, t13571: F, t13574: F, t13577: F, t13580: F, t13585: F, t13587: F, t13589: F, t13592: F, t13595: F, t13600: F, t13603: F, t13645: F) -> (F, F, F, F, F) {
    let t14103 = t4546 * t4199;
    let t14105 = t4546 * t4207;
    let t14107 = t4589 * t515;
    let t14108 = F::new(8.0) / F::new(15.0) * t14107;
    let t14110 = t172 * t4645 * t184;
    let t14112 = F::new(4.0) / F::new(5.0) * t14110 * t496;
    let t14131 = -t10043 + F::new(0.04534) * t13568 - F::new(0.04534) * t13571 - F::new(0.02518888888888889) * t13574 + F::new(0.04534) * t13577 - F::new(0.06801) * t13580 - F::new(0.0012594444444444445) * t13585 - F::new(0.002099074074074074) * t13587 + F::new(0.02770777777777778) * t13589 + F::new(0.0012594444444444445) * t13592 - F::new(0.007556666666666666) * t13595 + F::new(0.005597530864197531) * t13600 - F::new(0.012594444444444445) * t13603 - F::new(0.005037777777777778) * t10092 + F::new(0.0016792592592592592) * t10098 - F::new(0.0006996913580246914) * t10100 + F::new(0.002518888888888889) * t10115 + F::new(0.034005) * t13645 + F::new(0.002518888888888889) * t10053;
    (t14103, t14105, t14108, t14112, t14131)
}
