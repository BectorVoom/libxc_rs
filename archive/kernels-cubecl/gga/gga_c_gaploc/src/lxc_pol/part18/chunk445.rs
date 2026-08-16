//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 445/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk445<F: Float>(t1512: F, t325: F, t61: F, t1710: F, t836: F, t568: F, t808: F, t679: F, t685: F, t806: F, t1835: F, t1716: F, t531: F) -> (F, F, F, F, F, F, F) {
    let t2166 = t61 * t1512 * t325;
    let t2169 = t836 * t1710;
    let t2170 = t568 * t2169;
    let t2173 = t808 * t1710;
    let t2174 = t568 * t2173;
    let t2177 = t679 * t685;
    let t2178 = t2177 * t806;
    let t2181 = t808 * t1835;
    let t2182 = t568 * t2181;
    let t2185 = t531 * t1716;
    (t2166, t2170, t2174, t2177, t2178, t2182, t2185)
}
