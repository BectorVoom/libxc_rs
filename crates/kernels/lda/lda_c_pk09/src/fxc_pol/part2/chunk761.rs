//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 761/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk761<F: Float>(t2275: F, t748: F, t2279: F, t62: F, t7704: F, t903: F, t890: F, t2152: F, t650: F, t891: F, t61: F, t7831: F, t2143: F, t844: F, t164: F, t7766: F) -> (F, F, F, F, F, F, F) {
    let t8849 = t748 * t2275;
    let t8851 = t748 * t2279;
    let t8857 = t62 * t7704;
    let t8858 = t903 * t8857;
    let t8859 = t890 * t8858;
    let t8861 = t650 * t2152;
    let t8862 = t891 * t8861;
    let t8863 = t890 * t8862;
    let t8865 = t61 * t7831;
    let t8866 = t891 * t8865;
    let t8867 = t890 * t8866;
    let t8869 = t650 * t2143;
    let t8870 = t844 * t8869;
    let t8871 = t164 * t8870;
    let t8873 = t61 * t7766;
    (t8849, t8851, t8859, t8863, t8867, t8871, t8873)
}
