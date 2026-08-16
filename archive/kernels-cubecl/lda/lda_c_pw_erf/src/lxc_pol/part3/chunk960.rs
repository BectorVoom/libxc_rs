//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 960/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk960<F: Float>(t1179: F, t1738: F, t1171: F, t10764: F, t1187: F, t168: F, t4079: F, t635: F, t1089: F, t1125: F, t153: F, t274: F, t8798: F) -> (F, F, F, F, F, F) {
    let t10991 = F::cast_from(0.31931290694012293_f64) * t1738 * t1179;
    let t10992 = t1738 * t1171;
    let t10995 = F::cast_from(0.0012955432484775182_f64) * t10764 * t1187;
    let t10999 = t168 * t635 * t4079;
    let t11002 = t153 * t1125 * t1089;
    let t11006 = F::cast_from(19.1926369973667_f64) * t153 * t8798 * t274;
    (t10991, t10992, t10995, t10999, t11002, t11006)
}
