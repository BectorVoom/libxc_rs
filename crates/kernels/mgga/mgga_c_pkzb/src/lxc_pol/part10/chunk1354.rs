//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1354/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1354<F: Float>(t10063: F, t8273: F, t10116: F, t3174: F, t68: F, t931: F, t9795: F, t10071: F, t3206: F, t926: F, t10102: F, t8450: F, t10115: F, t10261: F, t2185: F, t2226: F, t22991: F, t23007: F, t23010: F, t23013: F, t23020: F, t23028: F, t2888: F, t824: F, t8282: F) -> (F,) {
    let t26986 = t10063 * t8273;
    let t26995 = t3174 * t68 * t10116;
    let t27001 = t931 * t9795;
    let t27007 = t3206 * t926 * t10071;
    let t27014 = t8450 * t926 * t10102;
    let t27017 = -0.57165357490759649296e-3 * t22991 - 2.0 / 27.0 * t26986 + t10063 * t8282 / 3.0 + t3174 * t2888 * t10261 * t2226 / 4.0 + t26995 / 72.0 + t3174 * t2888 * t10115 * t2185 / 48.0 + t3174 * t2888 * t27001 * t824 / 24.0 - 0.57165357490759649296e-3 * t27007 - t23007 / 54.0 + t23010 / 72.0 - t23013 / 24.0 + t23020 / 36.0 + 0.28582678745379824648e-3 * t27014 - 0.17149607247227894789e-2 * t23028;
    (t27017,)
}
