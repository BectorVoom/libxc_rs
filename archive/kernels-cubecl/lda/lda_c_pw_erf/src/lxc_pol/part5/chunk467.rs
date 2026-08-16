//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 467/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk467<F: Float>(t2260: F, t647: F, t656: F, t851: F, t1219: F, t1220: F, t1922: F, t1923: F, t1936: F, t1937: F, t1939: F, t1962: F, t1984: F, t1986: F, t1989: F, t1994: F, t1999: F, t2004: F, t2253: F, t2257: F, t256: F) -> (F, F, F) {
    let t2261 = t2260 * t647;
    let t2263 = t851 * t656;
    let t2265 = t1219 - F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t1220 - t1922 - t1923 + t2253 * t256 / F::cast_from(3.0_f64) + t2257 / F::cast_from(3.0_f64) + F::cast_from(0.06077777777777778_f64) * t2261 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t2263 + t1936 + t1937 + t1939 + t1962 + t1984 + t1986 - t1989 + t1994 - t1999 + t2004;
    (t2261, t2263, t2265)
}
