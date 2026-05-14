//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1252/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1252<F: Float>(t4763: F, t5367: F, t518: F, t6610: F, t577: F, t1480: F, t7007: F, t1488: F, t5363: F, t1318: F, t2151: F, t219: F, t558: F, t811: F, t12571: F, t12600: F, t2176: F, t519: F, t806: F) -> (F, F, F, F, F, F, F, F) {
    let t18607 = 8.0 / 15.0 * t4763 * t5367;
    let t18608 = t6610 * t518;
    let t18610 = 16.0 / 45.0 * t18608 * t577;
    let t18612 = 8.0 / 45.0 * t7007 * t1480;
    let t18614 = 8.0 / 27.0 * t7007 * t1488;
    let t18615 = t4763 * t5363;
    let t18616 = 32.0 / 45.0 * t18615;
    let t18621 = 32.0 / 45.0 * t1318 * t2151 * t219 * t811 * t558;
    let t18623 = 16.0 / 45.0 * t1318 * t12571;
    let t18627 = 16.0 / 45.0 * t519 * t2176 * t12600 * t806;
    (t18607, t18610, t18612, t18614, t18616, t18621, t18623, t18627)
}
