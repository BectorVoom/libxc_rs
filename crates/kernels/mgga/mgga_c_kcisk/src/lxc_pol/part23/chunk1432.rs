//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1432/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1432<F: Float>(t32457: F, t964: F, t1310: F, t1589: F, t21992: F, t32441: F, t109518: F, t109756: F, t114487: F, t114490: F, t114505: F, t114510: F, t114520: F, t115162: F, t115165: F, t19033: F, t19087: F, t21478: F, t32436: F, t32459: F, t32464: F, t33762: F, t33771: F, t33906: F, t33914: F, t33923: F, t33925: F, t9536: F) -> (F, F) {
    let t115750 = t964 * t32457;
    let t115772 = t1310 * t1589;
    let t115774 = t115772 * t21992 * t32441;
    let t115782 = 0.25794135802469135802e-2 * t114487 + 0.69444444444444444444e-2 * t9536 * t115750 * t32459 * t21478 + 0.69444444444444444444e-2 * t9536 * t115165 + 0.13888888888888888889e-1 * t9536 * t115162 * t33914 * t19087 - 0.46296296296296296296e-2 * t32436 * t33925 + 0.13402777777777777778e-2 * t109518 * t33771 + 0.69644166666666666666e-2 * t114490 + 0.34722222222222222222e-2 * t32436 * t33906 - 0.10416666666666666667e-1 * t9536 * t32464 * t33923 * t19033 + 0.20833333333333333334e-1 * t9536 * t115774 + 0.17411041666666666666e-2 * t114505 - 0.23214722222222222222e-2 * t114510 + 0.23214722222222222222e-2 * t114520 + 0.10722222222222222222e-1 * t109756 * t33762;
    (t115774, t115782)
}
