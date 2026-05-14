//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1035/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1035<F: Float>(t20668: F, t20709: F, t20758: F, t20790: F, t1387: F, t2181: F, t2192: F, t3812: F, t1354: F, t19221: F, t14082: F, t5650: F, t1346: F, t5704: F, t1349: F, t3831: F) -> (F, F, F, F, F, F, F) {
    let t20792 = t20668 + t20709 + t20758 + t20790;
    let t20796 = t1387 * t2181;
    let t20798 = t3812 * t2192;
    let t20800 = t1354 * t19221;
    let t20803 = t14082 * t5650;
    let t20806 = 0.93706135855523581992e-2 * t1346 * t5704;
    let t20812 = t1349 * t3831;
    (t20792, t20796, t20798, t20800, t20803, t20806, t20812)
}
