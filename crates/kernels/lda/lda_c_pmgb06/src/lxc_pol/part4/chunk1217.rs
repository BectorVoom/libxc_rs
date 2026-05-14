//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1217/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1217<F: Float>(t16750: F, t16752: F, t16755: F, t16757: F, t16759: F, t16773: F, t16775: F, t16780: F, t16781: F, t16782: F, t16783: F, t16785: F, t16786: F, t16787: F, t16788: F, t16789: F, t16790: F, t16797: F, t16800: F, t16801: F, t16806: F, t16809: F, t16812: F, t16817: F, t16820: F, t16824: F, t16828: F, t16833: F, t16835: F, t16836: F) -> (F, F) {
    let t18264 = t16750 + t16752 + t16755 + t16757 + t16759 + t16773 + t16775 + t16780 + t16781 - t16782 + t16783 - t16785 + t16786 - t16787 + t16788;
    let t18265 = t16789 - t16790 + t16797 + t16800 + t16801 + t16806 - t16809 - t16812 - t16817 + t16820 + t16824 + t16828 + t16833 - t16835 - t16836;
    (t18264, t18265)
}
