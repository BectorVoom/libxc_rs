//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1048/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1048<F: Float>(t14050: F, t6343: F, t3375: F, t5727: F, t1163: F, t4680: F, t5726: F, t1180: F, t1181: F, t12931: F, t12933: F, t12939: F, t12943: F, t1552: F, t16044: F, t16051: F, t16055: F, t360: F, t4298: F, t5989: F, t6151: F) -> (F,) {
    let t21049 = t14050 * t6343;
    let t21052 = t3375 * t5727;
    let t21055 = t1163 * t4680 * t5726;
    let t21057 = 0.34299214494455789578e-2 * t1180 * t1181 * t4298 * t5989 + 0.34299214494455789578e-2 * t1180 * t1181 * t1552 * t6151 * t360 + 0.34299214494455789578e-2 * t16044 + 0.80031500487063509016e-2 * t12931 - 0.80031500487063509016e-2 * t12933 + 0.51448821741683684367e-2 * t12939 + 0.34299214494455789578e-2 * t12943 - 0.13719685797782315831e-1 * t16051 - 0.85748036236139473944e-2 * t21049 - 0.68598428988911579156e-2 * t16055 + 0.17149607247227894789e-2 * t21052 + 0.17149607247227894789e-2 * t21055;
    (t21057,)
}
