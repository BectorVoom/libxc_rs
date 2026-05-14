//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1164/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1164<F: Float>(t6923: F, t8232: F, t6858: F, t1882: F, t28260: F, t28438: F, t108057: F, t108088: F, t108101: F, t14127: F, t14140: F, t14196: F, t1901: F, t2373: F, t24569: F, t24669: F, t24773: F, t2526: F, t2574: F, t265: F, t3281: F, t3842: F, t3977: F, t446: F, t6154: F, t67847: F, t6837: F, t6940: F, t724: F, t729: F, t762: F, t97928: F, t98001: F) -> (F,) {
    let t111109 = t8232 * t6923;
    let t111111 = t8232 * t6858;
    let t111121 = 4.0 / 9.0 * t1882 * t28260;
    let t111137 = 4.0 / 9.0 * t1882 * t28438;
    let t111150 = -2.0 / 3.0 * t446 * t2574 * t6154 * t14140 - 2.0 / 9.0 * t3281 * t724 * t762 * t24569 + 4.0 / 27.0 * t111109 - 4.0 / 27.0 * t111111 - 4.0 / 3.0 * t1901 * t14127 * t97928 * t3842 - 4.0 / 3.0 * t1901 * t67847 * t24669 - t111121 + 2.0 / 3.0 * t1901 * t14196 * t108057 + 2.0 / 27.0 * t98001 - 2.0 / 3.0 * t446 * t2574 * t762 * t6940 * t2373 + t446 * t729 * t762 * t6837 * t2526 / 3.0 - t111137 + t446 * t729 * t3977 * t24773 / 3.0 + 2.0 / 3.0 * t446 * t2574 * t265 * t108088 + 2.0 / 3.0 * t446 * t2574 * t265 * t108101;
    (t111150,)
}
