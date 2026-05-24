//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 986/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk986<F: Float>(t14535: F, t493: F, t1483: F, t4309: F, t1492: F, t4174: F, t486: F, t1497: F, t4297: F, t13329: F, t492: F, t13331: F, t499: F) -> (F, F, F, F, F, F) {
    let t14536 = t493 * t14535;
    let t14538 = t1483 * t4309;
    let t14540 = t1492 * t4174;
    let t14541 = t486 * t14540;
    let t14543 = t4297 * t1497;
    let t14545 = t13329 * t492;
    let t14546 = t499 * t13331;
    (t14536, t14538, t14541, t14543, t14545, t14546)
}
