//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 701/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk701<F: Float>(t5277: F, t718: F, t5291: F, t10436: F, t7303: F, t7302: F, t11236: F, t740: F, t5317: F, t1931: F, t5299: F, t11225: F, t732: F, t11228: F, t719: F, t735: F, sigma2: F) -> (F, F, F, F, F, F) {
    let t11763 = t5277 * t718;
    let t11764 = t11763 * t5291;
    let t11766 = t7303 * t10436;
    let t11767 = t7302 * t11766;
    let t11769 = t11236 * t740;
    let t11770 = t11769 * t5317;
    let t11772 = t1931 * t5299;
    let t11774 = t732 * t11225;
    let t11775 = t11774 * sigma2;
    let t11776 = t719 * t11228;
    let t11777 = t735 * t11776;
    (t11764, t11767, t11770, t11772, t11775, t11777)
}
