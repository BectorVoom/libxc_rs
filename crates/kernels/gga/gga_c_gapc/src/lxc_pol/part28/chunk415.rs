//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 415/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk415<F: Float>(t2255: F, t772: F, t2132: F, t2201: F, t2209: F, t2213: F, t2217: F, t2221: F, t2225: F, t2229: F, t2234: F, t2238: F, t2239: F, t2242: F, t2246: F, t2253: F, t771: F, t794: F, t799: F, t802: F, t828: F, t832: F) -> (F,) {
    let t2256 = t772 * t2255;
    let t2259 = t2132 + 0.2740028945738165176e-4 * t828 * t2201 + 0.39958755458681575483e-5 * t2209 * t2213 - 0.77948343448359322927e-4 * t2217 * t802 - 0.91334298191272172533e-4 * t2221 * t832 + 0.33406432906439709826e-4 * t2225 * t802 + 0.2740028945738165176e-4 * t2229 * t832 + 0.56366309740899397906e-3 * t771 * t2234 + 0.18788769913633132635e-4 * t2238 * t2239 + 0.18788769913633132635e-4 * t794 * t2242 + 0.33406432906439709826e-4 * t799 * t2246 + 0.56366309740899397906e-3 * t2253 * t2256;
    (t2259,)
}
