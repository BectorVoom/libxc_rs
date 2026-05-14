//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 622/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk622<F: Float>(t352: F, t593: F, t3976: F, t549: F, t3974: F, t2017: F, t3610: F, t571: F, t1410: F, t640: F, t653: F, t256: F, t3929: F, t3935: F, t3938: F, t3940: F, t3944: F, t3947: F, t3950: F, t3951: F, t3955: F, t3957: F, t3959: F, t3960: F, t3963: F, t3972: F) -> (F, F, F, F, F, F, F, F) {
    let t3977 = t352 * t593;
    let t3979 = t3976 * t3977 * t549;
    let t3981 = 16.0 / 15.0 * t3974 * t3979;
    let t3982 = t2017 * t3610;
    let t3984 = 4.0 / 9.0 * t571 * t3982;
    let t3985 = t640 * t1410;
    let t3988 = 2.0 / 9.0 * t653 * t1410;
    let t3989 = t3929 + t3935 - t3938 + t3940 * t256 / 3.0 + t3944 + 0.18233333333333332 * t3947 + t3950 + 0.36466666666666664 * t3951 + t3955 + t3957 - t3959 + 0.09973633333333333 * t3960 + t3963 - t3972 - t3981 + t3984 - 2.0 / 9.0 * t3985 - t3988;
    (t3977, t3979, t3981, t3982, t3984, t3985, t3988, t3989)
}
