//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1086/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1086<F: Float>(t7218: F, t7580: F, t9207: F, t1772: F, t12254: F, t9176: F, t2023: F, t7261: F, t7632: F, t7638: F, t18254: F, t18264: F, t18270: F, t18275: F, t18323: F, t2013: F, t2016: F, t5471: F, t7591: F, t7634: F, t9173: F, sigma2: F) -> (F,) {
    let t24876 = t7580 * t7218;
    let t24879 = t9207 * sigma2;
    let t24880 = t24879 * t1772;
    let t24885 = t12254 * t9176;
    let t24886 = t24885 * t2023;
    let t24887 = t7261 * t24886;
    let t24890 = t7632 * t7638;
    let t24891 = t7261 * t24890;
    let t24896 = -0.28785261945883707542e0 * t7591 * t7634 - 0.47975436576472845903e-1 * t24876 * t2016 + 0.89953943580886586067e-2 * t24880 * t2016 + 0.11993859144118211476e-1 * t5471 * t9173 - 0.16191709844559585492e0 * t2013 * t24887 + 0.10794473229706390328e0 * t2013 * t24891 - 0.11993859144118211476e-1 * t18254 - t18264 + t18270 + 0.59969295720591057377e-2 * t18275 + t18323;
    (t24896,)
}
