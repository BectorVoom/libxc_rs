//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 857/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk857<F: Float>(t1543: F, t277: F, t495: F, t360: F, t279: F, t5170: F, t5175: F, t5179: F, t5183: F, t5186: F, t5189: F, t6055: F, t6062: F, t6066: F, t6073: F, t6075: F, t6084: F, t6089: F, t6095: F, t6097: F, t6105: F, t6106: F) -> (F, F, F, F) {
    let t6107 = t277 * t1543;
    let t6108 = t6107 * t495;
    let t6109 = t360 * t6108;
    let t6112 = 0.82318114786693894983e-2 * t5170 - 0.16463622957338778996e-1 * t5175 + 0.2037639021386884617e0 * t5179 + 0.6112917064160653851e0 * t5183 - 0.34930954652346593433e-1 * t5186 - 0.1047928639570397803e0 * t5189 + 0.43341108700271342816e-1 * t6055 * t279 - t6062 + 0.58544643236296698111e-1 * t6066 + 0.48787202696913915093e-3 * t6073 + 0.19043987679069580388e-1 * t6075 - t6084 + 0.34930954652346593433e-1 * t6089 + 0.1047928639570397803e0 * t6095 + 0.12713391885412927226e1 * t6097 - t6105 - 0.15602799132097683414e1 * t6106 * t6109;
    (t6107, t6108, t6109, t6112)
}
