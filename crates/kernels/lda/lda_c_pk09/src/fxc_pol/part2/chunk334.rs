//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 334/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk334<F: Float>(t1579: F, t314: F, t306: F, t1243: F, t1255: F, t1263: F, t1272: F, t1251: F, t1259: F, t1268: F, t1275: F, t323: F) -> (F, F, F, F, F, F, F, F) {
    let t1580 = t314 * t1579;
    let t1581 = t1580 * t306;
    let t1584 = F::cast_from(1.5323028051206833_f64) * t1243;
    let t1586 = F::cast_from(0.5107676017068944_f64) * t1255;
    let t1588 = F::cast_from(0.3056501876701794_f64) * t1263;
    let t1590 = F::cast_from(0.1018833958900598_f64) * t1272;
    let t1592 = t1584 - F::cast_from(1.5323028051206833_f64) * t1251 + t1586 + F::cast_from(1.5323028051206833_f64) * t1259 + t1588 - F::cast_from(0.3056501876701794_f64) * t1268 + t1590 + F::cast_from(0.3056501876701794_f64) * t1275;
    let t1593 = t323 * t1592;
    (t1580, t1581, t1584, t1586, t1588, t1590, t1592, t1593)
}
