//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 902/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk902<F: Float>(t1459: F, t15167: F, t3018: F, t1460: F, t5186: F, t2993: F, t1484: F, t5218: F, t1483: F, t15374: F, t1472: F, t5154: F, t1471: F, t15401: F, t11671: F, t14885: F, t14887: F, t14889: F, t17338: F, t17342: F, t17346: F, t17350: F, t17354: F, t17358: F, t8871: F) -> (F, F, F, F, F, F, F, F, F) {
    let t17748 = t15167 * t1459;
    let t17750 = 0.48245472966453314466e2 * t3018 * t17748;
    let t17751 = t1460 * t5186;
    let t17753 = 6.0 * t2993 * t17751;
    let t17755 = t1484 * t5218;
    let t17758 = t15374 * t1483;
    let t17761 = t1472 * t5154;
    let t17764 = t15401 * t1471;
    let t17777 = -t8871 - 0.2283111111111111111e-1 * t11671 + 0.11415555555555555555e-1 * t14885 - 0.34246666666666666665e-1 * t14887 + 0.17123333333333333333e-1 * t14889 - 0.19025925925925925925e-1 * t17338 + 0.68493333333333333331e-1 * t17342 - 0.34246666666666666665e-1 * t17346 - 0.10274e0 * t17350 + 0.10274e0 * t17354 - 0.17123333333333333333e-1 * t17358;
    (t17748, t17750, t17751, t17753, t17755, t17758, t17761, t17764, t17777)
}
