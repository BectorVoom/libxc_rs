//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 784/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk784(t15904: f64, t24432: f64, t2075: f64, t2363: f64, t113: f64, t12823: f64, t1983: f64, t2040: f64, t2096: f64, t22574: f64, t22607: f64, t2312: f64, t2314: f64, t2320: f64, t23958: f64, t24008: f64, t24026: f64, t24028: f64, t24167: f64, t24169: f64, t24176: f64, t24428: f64, t4034: f64, t510: f64, t574: f64, t650: f64, t652: f64, t6876: f64, t7050: f64, t7057: f64, t7156: f64, t7171: f64, t7218: f64, t7220: f64) -> (f64, f64, f64) {
    let t24433 = t24432 * t15904;
    let t24442 = t2075 * t2363;
    let t24446 = 6.0_f64 * t1983 * t23958 - 2.0_f64 * t6876 * t7220 + t24026 * t574 - 2.0_f64 * t1983 * t24028 + t1983 * t24167 + 2.0_f64 * t1983 * t24169 - 2.0_f64 * t650 * t7156 - t2312 * t2075 + 6.0_f64 * t1983 * t24176 + 6.0_f64 * t6876 * t7171 + t22607 * t2096 + 2.0_f64 * t6876 * t7218 - t113 * t24428 - 2.0_f64 * t2320 * t2075 - 6.0_f64 * t22574 * t24433 - 4.0_f64 * t2314 * t7057 - 2.0_f64 * t12823 * t2040 - 4.0_f64 * t4034 * t7050 - 2.0_f64 * t652 * t24442 - t24008 * t510;
    (t24433, t24442, t24446)
}
