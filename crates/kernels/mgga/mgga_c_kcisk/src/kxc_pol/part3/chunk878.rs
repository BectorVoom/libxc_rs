//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 878/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk878<F: Float>(t14527: F, t487: F, t486: F, t13944: F, t6369: F, t6368: F, t13423: F, t381: F, t498: F, t493: F, t1483: F, t4309: F, t1492: F, t4174: F, t1497: F, t4297: F) -> (F, F, F, F, F, F) {
    let t14528 = t487 * t14527;
    let t14529 = t486 * t14528;
    let t14531 = t6369 * t13944;
    let t14532 = t6368 * t14531;
    let t14534 = t381 * t13423;
    let t14535 = t498 * t14534;
    let t14536 = t493 * t14535;
    let t14538 = t1483 * t4309;
    let t14540 = t1492 * t4174;
    let t14541 = t486 * t14540;
    let t14543 = t4297 * t1497;
    (t14529, t14532, t14536, t14538, t14541, t14543)
}
