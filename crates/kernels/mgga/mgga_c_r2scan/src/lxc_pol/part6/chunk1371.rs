//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1371/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1371<F: Float>(t19890: F, t6085: F, t7922: F, t2155: F, t25307: F, t6063: F, t6535: F, t8089: F, t6407: F, t8090: F, t2294: F, t7461: F, t7462: F, t546: F, t9520: F, t8074: F) -> (F, F, F, F, F, F, F) {
    let t26007 = t6085 * t19890 * t7922;
    let t26008 = 0.2037639021386884617e0 * t26007;
    let t26015 = t2155 * t6063 * t25307;
    let t26018 = t6535 * t19890 * t8089;
    let t26020 = t6407 * t8090;
    let t26021 = 0.17563392970889009434e0 * t26020;
    let t26027 = t7461 * t2294 * t7462;
    let t26029 = t546 * t9520;
    let t26036 = t6407 * t8074;
    (t26008, t26015, t26018, t26021, t26027, t26029, t26036)
}
