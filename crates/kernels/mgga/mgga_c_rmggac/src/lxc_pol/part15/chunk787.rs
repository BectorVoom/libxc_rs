//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 787/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk787<F: Float>(t7487: F, t9723: F, t2010: F, t2011: F, t291: F, t5878: F, t935: F, t9719: F, t938: F, t1661: F, t8342: F, t2415: F, t5757: F, t5061: F, t38552: F, t38554: F, t38556: F, t44808: F, t44812: F, t44816: F, t44818: F, t44821: F, t44825: F, t44828: F) -> (F,) {
    let t44831 = t7487 * t9723;
    let t44835 = t2010 * t2011 * t5878 * t291;
    let t44838 = t2010 * t9719 * t935;
    let t44841 = t2010 * t9719 * t938;
    let t44844 = t2010 * t8342 * t1661;
    let t44847 = t2010 * t2415 * t5757;
    let t44850 = t2010 * t2415 * t5061;
    let t44853 = 0.25538759935978703638e-4 * t44808 - 0.1064114997332445985e-4 * t44812 - 0.31923449919973379548e-4 * t44816 - 0.54549323308490683456e-1 * t44818 - 0.72042316457491791906e-3 * t44821 + 0.60975299583150056628e-3 * t38552 - 0.72042316457491791906e-3 * t44825 - 0.72042316457491791906e-3 * t44828 + 0.60975299583150056628e-3 * t38554 + 0.19211284388664477842e-2 * t44831 - 0.36021158228745895953e-3 * t44835 - 0.36021158228745895953e-3 * t44838 - 0.36021158228745895953e-3 * t44841 - 0.72042316457491791906e-3 * t44844 - 0.72042316457491791906e-3 * t44847 - 0.72042316457491791906e-3 * t44850 - 0.70441376091769752087e-2 * t38556;
    (t44853,)
}
