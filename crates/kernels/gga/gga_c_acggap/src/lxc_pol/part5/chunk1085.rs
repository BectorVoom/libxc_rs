//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1085/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1085<F: Float>(t3409: F, t5895: F, t12727: F, t1755: F, t3382: F, t5792: F, t1090: F, t1165: F, t1173: F, t1180: F, t1181: F, t1444: F, t1449: F, t1531: F, t1532: F, t1552: F, t1567: F, t157: F, t15947: F, t16871: F, t16940: F, t16942: F, t1884: F, t1894: F, t3169: F, t3396: F, t3403: F, t372: F, t4099: F, t4463: F, t6263: F, t955: F) -> (F,) {
    let t21982 = t3409 * t5895;
    let t21994 = t12727 * t1755;
    let t22000 = t3382 * t5792;
    let t22011 = -0.17149607247227894789e-1 * t3403 * t1181 * t1884 * t3169 - 0.34299214494455789578e-2 * t1531 * t1165 * t1552 * t6263 * t372 + 0.40015750243531754508e-2 * t21982 - 0.42874018118069736972e-3 * t1180 * t1181 * t1894 * t955 + 0.45351183609335988442e-1 * t16940 + 0.17149607247227894789e-2 * t1173 * t1165 * t1532 * t157 * t4099 - 0.85748036236139473944e-3 * t21994 + 0.34299214494455789578e-1 * t4463 * t1181 * t1567 * t1444 - 0.17149607247227894789e-2 * t22000 + 0.13719685797782315831e-1 * t3396 * t1181 * t15947 * t1449 + 7.0 / 144.0 * t16942 - 0.10289764348336736873e0 * t16871 * t1181 * t1884 * t1090;
    (t22011,)
}
