//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 986/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk986<F: Float>(t20808: F, t549: F, t3974: F, t5166: F, t20809: F, t4506: F, t4522: F, t11914: F, t20813: F, t12475: F, t12963: F, t2396: F, t10030: F, t7752: F, t18184: F, t2010: F) -> (F, F, F, F, F, F, F) {
    let t20823 = t20808 * t549;
    let t20826 = 8.0 / 9.0 * t3974 * t5166 * t20823;
    let t20829 = 4.0 / 9.0 * t4506 * t4522 * t20809;
    let t20832 = 32.0 / 27.0 * t4506 * t11914 * t20813;
    let t20835 = 16.0 / 15.0 * t12475 * t12963 * t2396;
    let t20836 = t10030 * t7752;
    let t20837 = 32.0 / 45.0 * t20836;
    let t20840 = 8.0 / 15.0 * t3974 * t18184 * t2010;
    (t20823, t20826, t20829, t20832, t20835, t20837, t20840)
}
