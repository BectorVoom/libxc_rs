//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1233/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1233<F: Float>(t443: F, t5616: F, t102: F, t1568: F, t1844: F, t3251: F, t763: F, t1664: F, t1856: F, t411: F, t5549: F, t1852: F, t3222: F) -> (F, F, F, F, F, F) {
    let t14535 = t5616 * t443;
    let t14549 = F::new(17.53815) * t102 * t1844 * t1568;
    let t14552 = F::new(5.84605) * t102 * t763 * t3251;
    let t14555 = F::new(52.61445) * t102 * t1856 * t1664;
    let t14558 = F::new(17.53815) * t102 * t5549 * t411;
    let t14561 = F::new(70.1526) * t102 * t1852 * t3222;
    (t14535, t14549, t14552, t14555, t14558, t14561)
}
