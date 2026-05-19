//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 887/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk887<F: Float>(t285: F, t4130: F, t477: F, t1128: F, t1155: F, t343: F, t465: F, t159: F, t4137: F, t169: F, t274: F, t301: F, t8363: F) -> (F, F, F, F, F, F) {
    let t8827 = t4130 * t477 * t285;
    let t8831 = F::cast_from(0.008135887625008338_f64) * t1155 * t1128 * t285;
    let t8832 = t343 * t465;
    let t8834 = t8832 * t159 * t285;
    let t8838 = F::cast_from(0.026861343269868797_f64) * t4137 * t477 * t285;
    let t8842 = F::cast_from(5.240451065072324_f64) * t169 * t8363 * t274 * t301;
    (t8827, t8831, t8832, t8834, t8838, t8842)
}
