//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1392/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1392<F: Float>(t120490: F, t32439: F, t35011: F, t3973: F, t9536: F, t120498: F, t9516: F, t109494: F, t34949: F, t109756: F, t1163: F, t120082: F, t120139: F, t120144: F, t120154: F, t32433: F, t32458: F, t33906: F, t33911: F, t33937: F, t33941: F, t34945: F, t34955: F, t34988: F) -> (F,) {
    let t120583 = t32439 * t120490;
    let t120590 = t9536 * t3973 * t35011;
    let t120594 = t9516 * t120498;
    let t120605 = t9536 * t109494 * t34949;
    let t120609 = 0.34722222222222222222e-2 * t33941 * t33906 + 0.34722222222222222222e-2 * t33941 * t33911 - 0.35740740740740740741e-2 * t109756 * t34955 + 0.44675925925925925927e-3 * t120583 + 0.6701388888888888889e-3 * t32439 * t120082 + 0.26805555555555555556e-2 * t32439 * t120154 + 0.11574074074074074074e-2 * t120590 + 0.16083333333333333334e-1 * t32433 * t34945 - 0.20104166666666666667e-2 * t120594 + 0.17361111111111111111e-2 * t9536 * t32458 * t34988 * t1163 + 0.80416666666666666668e-2 * t32439 * t120139 + 0.120625e-1 * t32439 * t120144 + 0.11574074074074074074e-2 * t120605 + 0.23280625e-2 * t33937 * t120144;
    (t120609,)
}
