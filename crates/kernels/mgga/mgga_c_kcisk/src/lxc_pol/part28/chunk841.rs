//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 841/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk841<F: Float>(t10487: F, t167: F, t11458: F, t1049: F, t695: F, t1849: F, t642: F, t1904: F, t5217: F, t1906: F, t724: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11630 = t167 * t10487;
    let t11633 = 0.71734315950379065738e-1 * t11458;
    let t11634 = t1049 * t695;
    let t11635 = 0.62154466893555682512e-3 * t11634;
    let t11682 = t642 * t1849;
    let t11694 = t1904 * t5217;
    let t11699 = t1906 * t1906;
    let t11700 = 1.0 / t11699;
    let t11701 = t724 * t11700;
    (t11630, t11633, t11634, t11635, t11682, t11694, t11699, t11700, t11701)
}
