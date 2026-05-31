//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 546/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk546<F: Float>(t1738: F, t688: F, t1550: F, t1727: F, t2806: F, t2809: F, t2811: F, t2822: F, t2828: F, t2831: F, t2835: F, t2836: F, t2838: F, t2841: F, t453: F) -> (F, F) {
    let t2842 = t1738 * t688;
    let t2844 = -F::cast_from(2.0_f64) * t453 * t2806 + F::cast_from(18.0_f64) * t2809 * t2811 + F::cast_from(2.0_f64) * t1727 * t1550 - t2822 + t2828 - F::cast_from(5.4655730795145296e-05_f64) * t2831 - t2835 + F::cast_from(0.05987117005127304_f64) * t2836 + F::cast_from(0.11974234010254609_f64) * t2838 - t2841 - F::cast_from(0.15965645347006147_f64) * t2842;
    (t2842, t2844)
}
