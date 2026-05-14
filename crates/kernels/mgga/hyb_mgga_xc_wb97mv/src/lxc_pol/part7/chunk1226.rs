//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1226/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1226<F: Float>(t10265: F, t2007: F, t554: F, t10269: F, t10274: F, t10258: F, t10250: F, t10254: F, t8474: F, t10515: F, t125: F, t3966: F, t667: F, t3869: F, t6432: F, t10273: F, t1983: F, t21747: F, t25394: F, t25397: F, t25403: F, t25406: F, t25421: F, t25424: F, t25428: F, t544: F, t557: F) -> (F,) {
    let t29812 = t554 * t2007 * t10265;
    let t29815 = t554 * t2007 * t10269;
    let t29819 = t554 * t2007 * t10274;
    let t29823 = t554 * t2007 * t10258;
    let t29826 = t554 * t2007 * t10250;
    let t29829 = t554 * t8474 * t10254;
    let t29831 = t10515 * t125;
    let t29836 = t3966 * t667;
    let t29847 = t554 * t6432 * t3869;
    let t29850 = -t554 * t557 * t1983 * t10273 / 32.0 - t29812 / 96.0 - t29815 / 96.0 - t25394 / 24.0 - t29819 / 96.0 - t25397 / 96.0 - t29823 / 48.0 - t29826 / 48.0 + 7.0 / 48.0 * t29829 - t554 * t557 * t29831 * t544 / 32.0 - t554 * t557 * t29836 * t544 / 32.0 - t25403 / 96.0 - t25406 / 72.0 - t25421 / 48.0 - t25424 / 24.0 + t25428 / 72.0 + t29847 / 144.0 - 5.0 / 144.0 * t21747;
    (t29850,)
}
