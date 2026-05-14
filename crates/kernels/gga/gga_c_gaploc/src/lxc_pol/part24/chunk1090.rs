//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1090/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1090<F: Float>(t32171: F, t10677: F, t1835: F, t10667: F, t325: F, t701: F, t10760: F, t7137: F, t723: F, t835: F, t10718: F, t10779: F, t10784: F, t10789: F, t1716: F, t1897: F, t1901: F, t2508: F, t2580: F, t32159: F, t32161: F, t32167: F, t32169: F, t7129: F) -> (F, F, F, F, F, F, F) {
    let t32172 = 0.64087718584518535698e-3 * t32171;
    let t32173 = t10677 * t1835;
    let t32179 = t325 * t10667;
    let t32180 = t32179 * t701;
    let t32185 = 0.6152420984113779427e-1 * t7137 * t10760;
    let t32186 = t32179 * t723;
    let t32190 = t835 * t10667;
    let t32191 = t32190 * t723;
    let t32201 = 0.41016139894091862846e-1 * t7137 * t10718;
    let t32202 = t32159 + t32161 - t32167 + t32169 + t32172 + 0.76905262301422242837e-2 * t1897 * t1901 * t32173 + 0.15381052460284448567e-1 * t7129 * t10779 + 0.15381052460284448567e-1 * t1897 * t1901 * t32180 - t32185 - 0.46143157380853345702e-1 * t2508 * t1901 * t32186 + 0.30762104920568897134e-1 * t2508 * t2580 * t32191 - 0.23071578690426672851e-1 * t2508 * t10789 * t1716 + 0.41016139894091862847e-1 * t7137 * t10784 + t32201;
    (t32173, t32179, t32180, t32186, t32190, t32191, t32202)
}
