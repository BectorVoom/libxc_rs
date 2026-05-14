//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 604/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk604<F: Float>(t12255: F, t313: F, t12223: F, t701: F, t1445: F, t1457: F, t10022: F, t10026: F, t10030: F, t10042: F, t11059: F, t11063: F, t11067: F, t11071: F, t11108: F, t11111: F, t11118: F, t11121: F, t12252: F, t2004: F, t2028: F, t2639: F, t807: F) -> (F, F, F) {
    let t12256 = t313 * t12255;
    let t12259 = t12223 * t701;
    let t12260 = t1445 * t12259;
    let t12263 = t1457 * t12259;
    let t12267 = -t11059 + t11063 - t11067 + t11071 + t11108 - t10022 - 0.39722766613167140743e-1 * t12252 * t2028 - 0.10725146985555128001e1 * t12256 * t2639 + 0.23005755572352449806e1 * t807 * t12260 + 0.35750489951850426669e0 * t2004 * t12263 - t10026 - 0.51123901271894332903e0 * t10030 + t10042 - t11111 - t11118 + t11121;
    (t12256, t12259, t12267)
}
