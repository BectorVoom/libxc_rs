//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 850/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk850(t7498: f64, t7526: f64, t7544: f64, t7573: f64, t7629: f64, t7731: f64, t7814: f64, t7848: f64, t41: f64, t7337: f64, t163: f64, t164: f64, t169: f64, t171: f64, t4260: f64, t4265: f64, t4275: f64, t4279: f64, t5442: f64, t5444: f64, t5449: f64, t5468: f64, t5472: f64, t5479: f64) -> (f64, f64, f64) {
    let t7851 = t7498 + t7526 + t7544 + t7573 + t7629 + t7731 + t7814 + t7848;
    let t7856 = t41 * t7337;
    let t7862 = -t4260 - t4265 + t4275 - t4279 - 0.09451622166942335_f64 * t5444 + 0.1890324433388467_f64 * t5449 - 0.1890324433388467_f64 * t5442 - 0.005388405304614574_f64 * t169 * t171 * t7851 * t163 - 0.031505407223141116_f64 * t7856 * t164 - 0.0014862827083471494_f64 * t5468 + 0.01975389032890948_f64 * t5472 - 0.01185233419734569_f64 * t5479;
    (t7851, t7856, t7862)
}
