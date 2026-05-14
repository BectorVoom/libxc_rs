//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 995/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk995<F: Float>(t1806: F, t8501: F, t1060: F, t8491: F, t1814: F, t7715: F, t1824: F, t220: F, t2477: F, t6734: F, t6790: F, t16117: F, t2487: F, t8497: F, t7718: F, t11480: F, t8500: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t23074 = t1806 * t8501;
    let t23077 = t8491 * t1060;
    let t23080 = t1814 * t7715;
    let t23081 = t23080 * t1824;
    let t23084 = t2477 * t220;
    let t23087 = t6734 * t6790;
    let t23090 = t16117 * t2487;
    let t23093 = t8497 * t1060;
    let t23096 = t1814 * t7718;
    let t23097 = t23096 * t1824;
    let t23100 = t8501 * t1060;
    let t23103 = t11480 * t8500;
    (t23074, t23077, t23081, t23084, t23087, t23090, t23093, t23097, t23100, t23103)
}
