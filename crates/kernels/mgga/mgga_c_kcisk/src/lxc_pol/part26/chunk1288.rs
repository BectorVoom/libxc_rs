//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1288/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1288<F: Float>(t20149: F, t33826: F, t9536: F, t109645: F, t9535: F, t42127: F, t539: F, t33940: F, t9511: F, t123: F, t2734: F, t33959: F, t109494: F, t33816: F, t12951: F, t1597: F) -> (F, F, F, F, F, F, F) {
    let t115090 = t9536 * t20149 * t33826;
    let t115104 = t109645 * t9535;
    let t115105 = t539 * t42127;
    let t115111 = t9511 * t33940;
    let t115118 = t2734 * t33959 * t123;
    let t115137 = 0.11574074074074074074e-2 * t9536 * t109494 * t33816;
    let t115157 = t1597 * t12951;
    (t115090, t115104, t115105, t115111, t115118, t115137, t115157)
}
