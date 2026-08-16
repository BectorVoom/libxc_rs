//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1360/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1360<F: Float>(t10976: F, t10980: F, t10983: F, t10987: F, t10988: F, t10991: F, t10995: F, t11574: F, t125: F, t14488: F, t1729: F, t19421: F, t19847: F, t19850: F, t19860: F, t19864: F, t19866: F, t2205: F, t23124: F, t23152: F, t23166: F, t23176: F, t23354: F, t2644: F, t4429: F, t454: F, t5783: F, t5925: F, t7881: F) -> F {
    let t23358 = -F::cast_from(5.4655730795145296e-05_f64) * t10976 - t10980 + F::cast_from(0.0001639671923854359_f64) * t10983 + t10987 - F::cast_from(0.15965645347006147_f64) * t10988 - t10991 - t10995 - F::cast_from(0.0008717022455366076_f64) * t19847 - F::cast_from(0.0017434044910732151_f64) * t19850 + F::cast_from(18.0_f64) * t23124 * t2205 - F::cast_from(18.0_f64) * t14488 * t19421 + F::cast_from(0.5945049527603057_f64) * t19860 + F::cast_from(0.004067943812504169_f64) * t19864 + F::cast_from(18.0_f64) * t1729 * t2644 * t454 * t5925 - F::cast_from(9.0_f64) * t11574 * t7881 - F::cast_from(9.0_f64) * t5783 * t19866 * t4429 + (t23152 + t23166 + t23176 + t23354) * t125;
    t23358
}
