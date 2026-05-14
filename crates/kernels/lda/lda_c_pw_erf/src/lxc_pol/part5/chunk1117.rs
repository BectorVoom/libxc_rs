//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1117/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1117<F: Float>(t479: F, t7856: F, t10749: F, t10750: F, t10755: F, t10757: F, t10760: F, t10766: F, t10775: F, t11629: F, t11631: F, t11633: F, t11643: F, t11644: F, t11652: F, t164: F, t18735: F, t20661: F) -> (F,) {
    let t23157 = t7856 * t479;
    let t23166 = -0.5670973300165402 * t11629 - 0.00035595929614954216 * t11631 - t10749 - 0.031505407223141116 * t20661 * t164 - 0.031505407223141116 * t23157 - 0.09451622166942335 * t11633 - t11643 + 0.5670973300165402 * t11644 - 0.09451622166942335 * t18735 - 0.031505407223141116 * t10750 - t10755 + 0.1890324433388467 * t10757 + t10760 + 0.2634331482256014 * t11652 - t10766 - 0.005926167098672845 * t10775;
    (t23166,)
}
