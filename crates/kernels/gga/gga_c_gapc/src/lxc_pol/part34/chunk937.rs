//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 937/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk937<F: Float>(t10099: F, t3568: F, t3846: F, t972: F, t1096: F, t3622: F, t2469: F, t3832: F, t7063: F, t1125: F, t3449: F, t12039: F, t12041: F, t12042: F, t12048: F, t12049: F, t12051: F, t12057: F, t12060: F, t12064: F, t12150: F) -> (F, F, F, F, F) {
    let t12281 = 2.0 * t10099 * t3568;
    let t12282 = t3846 * t972;
    let t12285 = t3622 * t1096;
    let t12287 = 2.0 * t2469 * t12285;
    let t12288 = t3832 * t972;
    let t12290 = 6.0 * t7063 * t12288;
    let t12291 = t1125 * t3449;
    let t12293 = 2.0 * t2469 * t12291;
    let t12294 = 2.0 * t12282 * t2469 - t12039 + t12041 + t12042 - t12048 + t12049 - t12051 - t12057 - t12060 - t12064 + t12150 + t12281 + t12287 - t12290 + t12293;
    (t12282, t12285, t12288, t12291, t12294)
}
