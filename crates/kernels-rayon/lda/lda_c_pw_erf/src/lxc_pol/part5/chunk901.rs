//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 901/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk901(t1128: f64, t1155: f64, t285: f64, t343: f64, t465: f64, t159: f64, t4137: f64, t477: f64, t169: f64, t274: f64, t301: f64, t8363: f64) -> (f64, f64, f64, f64, f64) {
    let t8831 = 0.008135887625008338_f64 * t1155 * t1128 * t285;
    let t8832 = t343 * t465;
    let t8834 = t8832 * t159 * t285;
    let t8838 = 0.026861343269868797_f64 * t4137 * t477 * t285;
    let t8842 = 5.240451065072324_f64 * t169 * t8363 * t274 * t301;
    (t8831, t8832, t8834, t8838, t8842)
}
