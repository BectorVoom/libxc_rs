//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 549/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk549<F: Float>(t169: F, t242: F, t2877: F, t465: F, t717: F, t1098: F, t632: F, t1102: F, t1143: F, t699: F, t703: F, t161: F, t2872: F) -> (F, F, F, F, F, F, F, F) {
    let t2880 = F::new(0.5188034422540342) * t169 * t2877 * t242;
    let t2881 = t717 * t465;
    let t2883 = t169 * t2881 * t242;
    let t2887 = F::new(0.42447554366239165) * t169 * t1098 * t632;
    let t2893 = t169 * t1102 * t632;
    let t2897 = F::new(0.15917832887339686) * t169 * t699 * t1143;
    let t2906 = t169 * t703 * t1143;
    let t2908 = t2872 * t161;
    (t2880, t2881, t2883, t2887, t2893, t2897, t2906, t2908)
}
