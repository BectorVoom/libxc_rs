//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 345/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk345<F: Float>(t1235: F, t2119: F, t1242: F, t1248: F, t1249: F, t2075: F, t1240: F, t1247: F, t2113: F, t1254: F, t1258: F, t1268: F, t1271: F) -> (F, F, F, F) {
    let t2120 = t1235 * t2119;
    let t2123 = t1242 * t2119;
    let t2126 = t1248 * t1249 * t2075;
    let t2128 = F::cast_from(0.1898925e1_f64) * t2120 - t1240 - F::cast_from(0.29896666666666666667e0_f64) * t2113 + F::cast_from(0.3071625e0_f64) * t2123 - t1247 - F::cast_from(0.16431333333333333333e0_f64) * t2126;
    let t2129 = t2128 * t1254;
    let t2133 = -t1258 - F::cast_from(0.92708333333333333333e-2_f64) * t2113;
    let t2141 = F::cast_from(0.258925e1_f64) * t2120 - t1268 - F::cast_from(0.301925e0_f64) * t2113 + F::cast_from(0.16504875e0_f64) * t2123 - t1271 - F::cast_from(0.16557e0_f64) * t2126;
    (t2128, t2129, t2133, t2141)
}
