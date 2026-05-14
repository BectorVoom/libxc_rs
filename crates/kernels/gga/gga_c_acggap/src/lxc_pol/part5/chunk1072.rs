//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1072/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1072<F: Float>(t1713: F, t922: F, t13761: F, t345: F, t1734: F, t3132: F, t13691: F, t13694: F, t16209: F, t16211: F, t16213: F, t21669: F, t21671: F, t21675: F, t21679: F, t21681: F, t21684: F, t21687: F) -> (F, F, F, F, F) {
    let t21689 = t1713 * t922;
    let t21691 = t345 * t13761 * t21689;
    let t21693 = t1734 * t922;
    let t21695 = t345 * t3132 * t21693;
    let t21700 = 0.5868e1 * t21669 - 0.3912e1 * t21671 - 0.22005e1 * t21675 + 0.1467e1 * t21679 - 0.1956e1 * t21681 + 0.1467e1 * t21684 + 0.7335e0 * t21687 + 0.8802e1 * t21691 - 0.22005e1 * t21695 + 0.978e0 * t16209 - 0.4564e1 * t16211 + 0.76066666666666666667e1 * t16213 - t13691 + t13694;
    (t21689, t21691, t21693, t21695, t21700)
}
