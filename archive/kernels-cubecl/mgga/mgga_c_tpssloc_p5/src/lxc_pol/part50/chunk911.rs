//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 911/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk911<F: Float>(t23122: F, t25064: F, t4166: F, t6620: F, t849: F, t1516: F, t23127: F, t4261: F, t6621: F, t23133: F, t7503: F, t838: F) -> (F, F, F, F, F, F) {
    let t25065 = t23122 * t25064;
    let t25068 = t4166 * t6620;
    let t25069 = t25068 * t849;
    let t25071 = t23127 * t1516;
    let t25073 = t6621 * t4261;
    let t25077 = t23133 * t1516;
    let t25080 = t7503 * t838;
    (t25065, t25069, t25071, t25073, t25077, t25080)
}
