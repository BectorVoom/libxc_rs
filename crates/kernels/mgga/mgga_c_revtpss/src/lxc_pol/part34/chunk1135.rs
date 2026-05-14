//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1135/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1135<F: Float>(t1558: F, t231: F, t6071: F, t23327: F, t25270: F, t23297: F, t23346: F, t7045: F, t23331: F, t23293: F, t23301: F, t27261: F, t23336: F, t23323: F, t106006: F, t106010: F, t106014: F, t106022: F, t92976: F, t98964: F) -> (F, F) {
    let t113163 = t6071 * t1558 * t231;
    let t113171 = t25270 * t23327;
    let t113173 = t25270 * t23297;
    let t113177 = t7045 * t23346;
    let t113180 = t25270 * t23331;
    let t113182 = t25270 * t23293;
    let t113184 = t27261 * t23301;
    let t113186 = t27261 * t23336;
    let t113188 = t25270 * t23323;
    let t113192 = 0.51448821741683684367e-2 * t113171 - 0.12862205435420921092e-2 * t113173 + 0.6098400337114239387e-3 * t106006 - 0.48018900292238105409e-1 * t106010 - 0.17149607247227894789e-2 * t113177 + 0.24009450146119052704e-1 * t106014 - 0.25724410870841842184e-1 * t113180 + 0.51448821741683684367e-2 * t113182 + 0.25724410870841842183e-2 * t113184 - 0.10289764348336736873e-1 * t113186 + 0.51448821741683684367e-2 * t113188 - 0.45732285992607719437e-3 * t98964 + t92976 + 0.15246000842785598468e-2 * t106022;
    (t113163, t113192)
}
