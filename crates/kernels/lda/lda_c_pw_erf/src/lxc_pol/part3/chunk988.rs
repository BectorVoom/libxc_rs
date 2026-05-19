//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 988/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk988<F: Float>(t142: F, t5548: F, t455: F, t10833: F, t10843: F, t11482: F, t11486: F, t11495: F, t11499: F, t11501: F, t11507: F, t1733: F, t1735: F, t1881: F, t2208: F, t2211: F, t2806: F, t4283: F, t452: F, t456: F, t5490: F, t5783: F, t776: F, t8751: F, t9130: F) -> (F, F) {
    let t11510 = t142 * t5548;
    let t11511 = t455 * t11510;
    let t11516 = F::new(6.0) * t4283 * t776 * t456 - F::cast_from(0.16213771438917426_f64) * t11482 + F::new(3.0) * t10843 * t2208 + F::new(9.0) * t1733 * t11486 - F::new(6.0) * t1881 * t2806 + F::new(9.0) * t2211 * t9130 - F::cast_from(0.0008717022455366076_f64) * t11495 - t11499 - F::cast_from(0.0008717022455366076_f64) * t11501 - F::cast_from(2.7743564462147594_f64) * t8751 + F::new(18.0) * t5490 * t452 * t2208 + F::new(9.0) * t11507 * t1735 + F::new(9.0) * t1733 * t11511 - F::new(18.0) * t5783 * t10833;
    (t11510, t11516)
}
