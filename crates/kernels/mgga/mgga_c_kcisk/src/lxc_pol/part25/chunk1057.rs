//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1057/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1057<F: Float>(t17841: F, t17844: F, t17848: F, t17850: F, t17853: F, t17857: F, t17859: F, t17863: F, t17866: F, t17869: F, t17872: F, t17876: F, t17879: F, t17883: F, t17886: F, t17889: F, t17892: F, t17895: F) -> (F,) {
    let t18882 = -0.16666666666666666667e0 * t17841 - 0.20234375e-1 * t17844 + 0.60703125e-1 * t17848 + 0.125e0 * t17850 - 0.4046875e-1 * t17853 + 0.25e0 * t17857 - 0.13489583333333333333e-1 * t17859 - 0.1875e0 * t17863 + 0.25e0 * t17866 + 0.375e0 * t17869 - 0.4046875e-1 * t17872 - 0.5625e0 * t17876 - 0.20833333333333333333e-1 * t17879 - 0.9375e-1 * t17883 - 0.625e-1 * t17886 - 0.41666666666666666666e-1 * t17889 - 0.53958333333333333333e-1 * t17892 + 0.26979166666666666666e-1 * t17895;
    (t18882,)
}
