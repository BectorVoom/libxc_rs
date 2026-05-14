//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 236/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk236<F: Float>(t721: F, t984: F, t570: F, t574: F, t666: F, t670: F, t612: F, t616: F, t626: F, t636: F, t653: F, t676: F, t681: F, t687: F, t101: F, t89: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t986 = 3.7610742193750633 * t984 * t721;
    let t993 = 0.008222864943561326 * t570;
    let t994 = 0.09983749558483038 * t574;
    let t995 = 2.2984542076810275 * t666;
    let t996 = 1.532302805120685 * t670;
    let t1000 = 0.15282509383508946 * t612;
    let t1001 = 0.10188339589005964 * t616;
    let t1005 = t995 + t996 + 2.2984542076810275 * t676 + 2.2984542076810275 * t681 - 2.2984542076810275 * t687 + t1000 + t1001 + 0.15282509383508946 * t626 + 0.15282509383508946 * t636 - 0.15282509383508946 * t653;
    let t1006 = t101 * t1005;
    let t1007 = t1006 * t89;
    (t986, t993, t994, t995, t996, t1000, t1001, t1005, t1006, t1007)
}
