//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1203/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1203<F: Float>(t2922: F, t5974: F, t7775: F, t2918: F, t5939: F, t757: F, t2082: F, t2939: F, t771: F, t7755: F, t1066: F, t179: F, t18107: F, t299: F, t2068: F, t7350: F) -> (F, F, F, F, F, F) {
    let t21643 = t2922 * t5974 * t7775;
    let t21651 = t757 * t5939 * t2918;
    let t21655 = t2082 * t2939;
    let t21657 = t771 * t7755;
    let t21661 = t299 * t179 * t18107 * t1066;
    let t21667 = t299 * t179 * t2068 * t7350;
    (t21643, t21651, t21655, t21657, t21661, t21667)
}
