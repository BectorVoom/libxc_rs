//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1213/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1213<F: Float>(t32168: F, t1843: F, t24474: F, t7064: F, t10677: F, t1835: F, t10667: F, t325: F, t701: F, t10760: F, t7137: F, t723: F) -> (F, F, F, F, F, F, F) {
    let t32169 = F::cast_from(0.64087718584518535698e-3_f64) * t32168;
    let t32171 = t7064 * t1843 * t24474;
    let t32172 = F::cast_from(0.64087718584518535698e-3_f64) * t32171;
    let t32173 = t10677 * t1835;
    let t32179 = t325 * t10667;
    let t32180 = t32179 * t701;
    let t32185 = F::cast_from(0.6152420984113779427e-1_f64) * t7137 * t10760;
    let t32186 = t32179 * t723;
    (t32169, t32172, t32173, t32179, t32180, t32185, t32186)
}
