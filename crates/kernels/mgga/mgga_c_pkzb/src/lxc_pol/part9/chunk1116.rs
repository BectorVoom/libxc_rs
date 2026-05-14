//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1116/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1116<F: Float>(t20917: F, t20957: F, t21002: F, t21040: F, t21121: F, t21171: F, t21218: F, t21258: F, t237: F, t5893: F, t730: F, t7535: F, t1306: F, t20824: F, t20827: F, t20831: F, t20892: F, t20895: F, t20898: F, t20900: F, t20902: F, t2997: F, t6058: F) -> (F, F, F) {
    let t21262 = t237 * (t20917 + t20957 + t21002 + t21040 + t21121 + t21171 + t21218 + t21258);
    let t21265 = 0.51947577317044391277e2 * t730 * t7535 * t5893;
    let t21266 = -t1306 * t2997 * t6058 - t20824 - t20827 + t20831 + t20892 - t20895 + t20898 + t20900 + t20902 + t21262 - t21265;
    (t21262, t21265, t21266)
}
