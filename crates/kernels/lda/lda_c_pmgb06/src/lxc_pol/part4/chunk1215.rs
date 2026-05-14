//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1215/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1215<F: Float>(t16606: F, t16608: F, t16609: F, t16610: F, t16613: F, t16617: F, t16619: F, t16621: F, t16623: F, t16625: F, t16629: F, t16631: F, t16633: F, t16635: F, t16637: F, t16640: F, t16649: F, t16652: F, t16654: F, t16657: F, t16659: F, t16662: F, t16665: F, t16668: F, t16671: F, t16673: F, t16676: F, t16679: F, t16682: F, t16685: F) -> (F, F) {
    let t18254 = -t16606 + t16608 + t16609 - t16610 + t16613 + t16617 + t16619 + t16621 + t16623 + t16625 + t16629 + t16631 + t16633 + t16635 + t16637;
    let t18255 = t16640 + t16649 + t16652 + t16654 + t16657 - t16659 - t16662 - t16665 - t16668 - t16671 + t16673 + t16676 + t16679 + t16682 + t16685;
    (t18254, t18255)
}
