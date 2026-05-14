//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1318/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1318<F: Float>(t17351: F, t17354: F, t17405: F, t17411: F, t17505: F, t20705: F, t25705: F, t25715: F, t25717: F, t25723: F, t25725: F, t25633: F, t25636: F, t25639: F, t25734: F, t25737: F, t25740: F, t25744: F, t25747: F, t25750: F, t25754: F, t25758: F, t25762: F) -> (F, F) {
    let t26083 = -0.3529725e1 * t25705 + 0.3529725e1 * t25715 + 0.6311625e0 * t25717 - 0.18523555555555555555e1 * t17405 + 0.34731666666666666666e0 * t17411 - 0.32136222222222222223e1 * t20705 + 0.10589175e2 * t25723 - 0.6311625e0 * t25725 + t17505 - 0.32136222222222222222e1 * t17351 + 0.68863333333333333333e0 * t17354;
    let t26096 = 0.68863333333333333333e0 * t25633 - 0.103295e1 * t25636 + 0.1549425e1 * t25639 + 0.34731666666666666667e0 * t25734 + 0.62517e0 * t25737 - 0.83356e0 * t25740 + 0.62517e0 * t25744 - 0.41678e0 * t25747 - 0.41678e0 * t25750 + 0.312585e0 * t25754 + 0.62517e0 * t25758 + 0.312585e0 * t25762;
    (t26083, t26096)
}
