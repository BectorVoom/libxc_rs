//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 664/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk664<F: Float>(t1710: F, t4474: F, t8051: F, t15648: F, t534: F, t25: F, t3066: F, t4491: F, t4455: F, t458: F, t4417: F, t7763: F, t363: F) -> (F, F, F, F, F, F, F) {
    let t15712 = t1710 * t4474;
    let t15716 = t8051 * t4474;
    let t15720 = t534 * t15648;
    let t15723 = t4474 * t25;
    let t15724 = t15723 * t3066;
    let t15727 = t1710 * t4491;
    let t15734 = t458 * t4455;
    let t15736 = t7763 * t4417;
    let t15737 = t15736 * t363;
    (t15712, t15716, t15720, t15724, t15727, t15734, t15737)
}
