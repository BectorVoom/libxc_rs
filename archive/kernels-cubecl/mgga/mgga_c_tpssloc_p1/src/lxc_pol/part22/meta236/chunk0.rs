//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1315/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1315<F: Float>(t153: F, t9862: F, t2371: F, t2531: F, t2528: F, t2517: F, t607: F, t707: F, t2652: F, t2663: F, t181: F, t686: F, t781: F) -> (F, F, F, F, F, F, F) {
    let t9863 = t153 * t9862;
    let t9864 = t2531 * t2371;
    let t9866 = t2531 * t2528;
    let t9868 = t2517 * t607;
    let t9869 = t707 * t9868;
    let t9871 = t2652 * t2663;
    let t9874 = t686 * t781 * t181;
    (t9863, t9864, t9866, t9868, t9869, t9871, t9874)
}
