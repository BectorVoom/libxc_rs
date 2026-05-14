//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 820/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk820<F: Float>(t1497: F, t5627: F, t1495: F, t1468: F, t1464: F, t3738: F, t5769: F, t1394: F, t3717: F, t5737: F, t1385: F, t1650: F, t2645: F) -> (F, F, F, F, F, F) {
    let t16673 = t5627 * t1497;
    let t16674 = t1495 * t16673;
    let t16675 = t1468 * t16674;
    let t16676 = t1464 * t16675;
    let t16678 = t3738 * t5769;
    let t16679 = t1394 * t16678;
    let t16681 = t5737 * t3717;
    let t16682 = t16681 * t1385;
    let t16685 = t1650 * t2645;
    (t16673, t16676, t16679, t16681, t16682, t16685)
}
