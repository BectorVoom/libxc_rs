//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1067/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1067<F: Float>(t24: F, t1429: F, t3374: F, t28885: F, t10523: F, t10528: F, t1430: F, t16250: F, t1651: F, t2548: F, t28895: F, t507: F, t6782: F, t6785: F, t8734: F, t8742: F, t91: F, zeta_threshold: F) -> (F, F, F) {
    let t90 = t24 <= zeta_threshold;
    let t28898 = t1429 * t3374;
    let t28906 = -t28885;
    let t28910 = piecewise3(t90, 0.0, 40.0 / 81.0 * t16250 * t10523 * t507 + 16.0 / 9.0 * t8734 * t1430 - 8.0 / 9.0 * t6782 * t28895 - 8.0 / 3.0 * t6785 * t28898 + 4.0 / 3.0 * t2548 * t8742 + 4.0 / 9.0 * t1651 * t10528 * t507 + 4.0 / 3.0 * t91 * t28906);
    (t28898, t28906, t28910)
}
