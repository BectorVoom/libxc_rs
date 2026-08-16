//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2599/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2599<F: Float>(t1734: F, t3507: F, t11721: F, t11786: F, t5005: F, t15730: F, t3536: F, t15594: F, t3523: F, t1174: F, t14726: F, t44562: F) -> (F, F, F, F, F, F) {
    let t52696 = t1734 * t3507;
    let t52704 = t1734 * t11721;
    let t52725 = t5005 * t11786;
    let t52731 = t3536 * t15730;
    let t52733 = t15594 * t3523;
    let t52751 = t1174 * t44562 * t14726;
    (t52696, t52704, t52725, t52731, t52733, t52751)
}
