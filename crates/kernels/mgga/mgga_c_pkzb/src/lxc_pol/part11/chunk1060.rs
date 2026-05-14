//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1060/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1060<F: Float>(t10437: F, t16089: F, t444: F, t1429: F, t3329: F, t8: F, t3333: F, t983: F, t1430: F, t2499: F, t8657: F, t10444: F, t1435: F, t27: F, t28676: F, t19530: F, t23870: F, t2504: F, t3347: F, t38: F, t6738: F, t8646: F, t8650: F, t8654: F, t8658: F, t991: F) -> (F, F, F, F, F, F, F, F) {
    let t28696 = t16089 * t10437 * t444;
    let t28700 = t3329 * t8 * t1429;
    let t28703 = t983 * t3333;
    let t28704 = t28703 * t444;
    let t28707 = t1430 * t3333;
    let t28710 = t2499 * t8657;
    let t28714 = t1435 * t10444 * t444;
    let t28718 = -t27 * t28676;
    let t28721 = -200.0 / 9.0 * t3347 * t2504 + 50.0 / 27.0 * t991 * t8646 + 100.0 / 9.0 * t23870 * t8650 - 50.0 / 9.0 * t991 * t8654 - 25.0 / 3.0 * t991 * t8658 + 40.0 / 81.0 * t38 * t28696 + 10.0 / 9.0 * t19530 * t28700 - 10.0 / 9.0 * t19530 * t28704 - 10.0 / 3.0 * t6738 * t28707 + 10.0 / 3.0 * t38 * t28710 + 10.0 / 9.0 * t38 * t28714 + 5.0 / 3.0 * t38 * t28718;
    (t28696, t28700, t28704, t28707, t28710, t28714, t28718, t28721)
}
