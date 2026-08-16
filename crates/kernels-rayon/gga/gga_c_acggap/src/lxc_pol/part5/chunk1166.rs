//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1166/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1166(t14050: f64, t6343: f64, t3375: f64, t5727: f64, t1163: f64, t4680: f64, t5726: f64, t1180: f64, t1181: f64, t12931: f64, t12933: f64, t12939: f64, t12943: f64, t1552: f64, t16044: f64, t16051: f64, t16055: f64, t360: f64, t4298: f64, t5989: f64, t6151: f64) -> f64 {
    let t21049 = t14050 * t6343;
    let t21052 = t3375 * t5727;
    let t21055 = t1163 * t4680 * t5726;
    let t21057 = 0.34299214494455789578e-2_f64 * t1180 * t1181 * t4298 * t5989 + 0.34299214494455789578e-2_f64 * t1180 * t1181 * t1552 * t6151 * t360 + 0.34299214494455789578e-2_f64 * t16044 + 0.80031500487063509016e-2_f64 * t12931 - 0.80031500487063509016e-2_f64 * t12933 + 0.51448821741683684367e-2_f64 * t12939 + 0.34299214494455789578e-2_f64 * t12943 - 0.13719685797782315831e-1_f64 * t16051 - 0.85748036236139473944e-2_f64 * t21049 - 0.68598428988911579156e-2_f64 * t16055 + 0.17149607247227894789e-2_f64 * t21052 + 0.17149607247227894789e-2_f64 * t21055;
    t21057
}
