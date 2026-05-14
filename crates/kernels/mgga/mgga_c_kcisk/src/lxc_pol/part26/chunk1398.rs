//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1398/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1398<F: Float>(t109475: F, t120630: F, t32464: F, t110106: F, t115871: F, t115941: F, t115950: F, t119636: F, t119645: F, t119654: F, t120632: F, t1586: F, t1589: F, t1597: F, t25342: F, t2737: F, t27801: F, t27817: F, t32439: F, t32458: F, t33914: F, t33923: F, t9536: F, t9855: F) -> (F,) {
    let t120769 = t32464 * t109475 * t120630;
    let t120789 = 0.23148148148148148148e-2 * t9536 * t120632 - 0.34722222222222222222e-2 * t9536 * t32458 * t33914 * t27817 - 0.34722222222222222222e-2 * t9536 * t120769 - 0.10416666666666666667e-1 * t9536 * t32464 * t33923 * t25342 - 0.13402777777777777778e-2 * t32439 * t120769 - 0.10722222222222222222e-1 * t115871 * t9855 - 0.92858888888888888885e-2 * t119636 - t115941 + 0.23214722222222222222e-2 * t119645 + 0.52083333333333333333e-2 * t2737 * t1586 * t1589 * t1597 * t27801 - 0.23214722222222222221e-2 * t119654 - 0.25794135802469135802e-3 * t110106 + t115950;
    (t120789,)
}
