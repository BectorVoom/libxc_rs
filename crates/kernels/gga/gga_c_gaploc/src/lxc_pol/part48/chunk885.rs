//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 885/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk885<F: Float>(t45108: F, t13548: F, t731: F, t11832: F, t22090: F, t2508: F, t7291: F, t13498: F, t13539: F, t1897: F, t1901: F, t43290: F, t43326: F, t43330: F, t45072: F, t45077: F, t45079: F, t45083: F, t45086: F, t45090: F, t45091: F, t45101: F, t45104: F, t45107: F, t7137: F) -> F {
    let t45109 = F::new(0.32043859292259267849e-3) * t45108;
    let t45110 = t731 * t13548;
    let t45111 = F::new(0.42725145723012357132e-3) * t45110;
    let t45115 = F::new(0.1845726295234133828e0) * t2508 * t22090 * t11832 * t7291;
    let t45116 = -t45072 - F::new(0.61524209841137794269e-1) * t7137 * t13498 - t45077 + t45079 + F::new(0.41016139894091862845e-1) * t7137 * t13539 + t45083 + t45086 + t45090 + F::new(0.76905262301422242837e-2) * t1897 * t1901 * t45091 - F::new(0.17090058289204942853e-2) * t43290 - F::new(0.1281754371690370714e-2) * t43326 - F::new(0.1281754371690370714e-2) * t43330 + t45101 + t45104 - t45107 + t45109 - t45111 + t45115;
    t45116
}
