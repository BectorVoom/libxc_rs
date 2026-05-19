//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 976/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk976<F: Float>(t14313: F, t1519: F, t1982: F, t1518: F, t2066: F, t211: F, t1131: F, t485: F, t5474: F, t1910: F, t2910: F, t1124: F, t1904: F, t483: F) -> (F, F, F, F, F, F) {
    let t14314 = F::new(8.0) / F::new(45.0) * t14313;
    let t14351 = t1982 * t1519;
    let t14352 = F::new(4.0) / F::new(45.0) * t14351;
    let t14365 = t211 * t1518 * t2066;
    let t14366 = F::new(4.0) / F::new(45.0) * t14365;
    let t14385 = t5474 * t1131 * t485;
    let t14386 = F::cast_from(0.01185233419734569_f64) * t14385;
    let t14388 = t1910 * t2910 * t485;
    let t14392 = t1124 * t1904 * t483 * t485;
    (t14314, t14352, t14366, t14386, t14388, t14392)
}
