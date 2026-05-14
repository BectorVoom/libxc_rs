//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 961/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk961<F: Float>(t1065: F, t675: F, t247: F, t906: F, t1063: F, t1062: F, t3223: F, t1052: F, t3147: F, t1036: F, t3141: F, t3144: F, t1035: F, t11239: F, t342: F, t3145: F, t334: F) -> (F, F, F, F, F, F, F, F) {
    let t11986 = t675 * t1065;
    let t11988 = t247 * t11986 * t906;
    let t11989 = t1063 * t11988;
    let t11994 = t3223 * t1062;
    let t11997 = t1052 * t3147;
    let t11998 = t1036 * t11997;
    let t11999 = t3141 * t11998;
    let t12012 = t3144 * t11997;
    let t12013 = t3141 * t12012;
    let t12046 = t11239 * t1035;
    let t12047 = t342 * t12046;
    let t12050 = 1.0 / t3145 / t334;
    (t11986, t11989, t11994, t11999, t12013, t12046, t12047, t12050)
}
