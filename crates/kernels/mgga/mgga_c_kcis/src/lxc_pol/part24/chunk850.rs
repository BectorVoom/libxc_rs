//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 850/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk850<F: Float>(t2822: F, t6630: F, t6625: F, t9438: F, t3200: F, t13155: F, t19396: F, t3210: F, t4554: F, t13131: F, t13130: F, t19399: F, t4555: F, t13199: F, t6626: F, t9429: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19802 = t2822 * t6630;
    let t19804 = t9438 * t6625;
    let t19805 = t3200 * t19804;
    let t19807 = t13155 * t19396;
    let t19808 = t3210 * t19807;
    let t19809 = t4554 * t19808;
    let t19811 = t13131 * t19396;
    let t19812 = t3210 * t19811;
    let t19813 = t13130 * t19812;
    let t19815 = t4555 * t19399;
    let t19816 = t3210 * t19815;
    let t19817 = t13199 * t19816;
    let t19819 = t9429 * t6626;
    (t19802, t19805, t19807, t19809, t19811, t19813, t19815, t19817, t19819)
}
