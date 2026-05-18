//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 997/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk997<F: Float>(t11636: F, t41: F, t479: F, t5451: F, t1590: F, t1905: F, t164: F, t4437: F, t10749: F, t10750: F, t10755: F, t10757: F, t10760: F, t11621: F, t11623: F, t11626: F, t11627: F, t11629: F, t11631: F, t11633: F) -> (F, F) {
    let t11637 = t41 * t11636;
    let t11640 = t5451 * t479;
    let t11642 = t1905 * t1590;
    let t11643 = F::new(0.09451622166942335) * t11642;
    let t11644 = t4437 * t164;
    let t11648 = -t11621 + F::new(0.09451622166942335) * t11623 + t11626 + F::new(0.09451622166942335) * t11627 - F::new(0.1890324433388467) * t11629 - F::new(0.00011865309871651405) * t11631 - t10749 - F::new(0.031505407223141116) * t11633 - F::new(0.031505407223141116) * t11637 * t164 - F::new(0.09451622166942335) * t11640 - t11643 + F::new(0.1890324433388467) * t11644 - F::new(0.09451622166942335) * t10750 - t10755 + F::new(0.5670973300165402) * t10757 + t10760;
    (t11637, t11648)
}
