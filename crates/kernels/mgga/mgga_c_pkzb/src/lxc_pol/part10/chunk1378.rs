//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1378/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1378<F: Float>(t2198: F, t2240: F, t3766: F, t18427: F, t18430: F, t18596: F, t22230: F, t22233: F, t22236: F, t27262: F, t27289: F, t27295: F, t378: F, t22639: F, t22699: F, t22762: F, t22826: F, t26854: F, t26859: F, t26861: F, t26863: F, t26865: F, t26867: F, t26869: F, t26871: F, t26873: F, t26875: F, t26878: F, t26883: F, t26892: F, t3088: F, t3107: F, t8139: F, t8161: F) -> (F, F, F) {
    let t27530 = 6.0 * t2240 * t3766 * t2198;
    let t27540 = (t18596 - 0.57685185185185185184e-1 * t18427 + 0.12361111111111111111e-1 * t18430 - 0.57685185185185185187e-1 * t22230 + 0.49444444444444444446e-1 * t22233 - 0.18541666666666666667e-1 * t22236 + 0.12361111111111111111e-1 * t27295 - 0.18541666666666666667e-1 * t27262 + 0.278125e-1 * t27289) * t378;
    let t27550 = -t26854 + t26859 - t26861 + t26863 - t26865 - t26867 + t26869 + t26871 + t26873 - t26875 + t26878 - t26883 - 0.19751673498613801407e-1 * t27540 + t26892 - 0.77193501593724168323e3 * t22639 * t8139 + 0.14035736694323150897e2 * t22699 * t8161 - 8.0 * t22826 * t3088 + 0.12865583598954028054e3 * t22762 * t3107;
    (t27530, t27540, t27550)
}
