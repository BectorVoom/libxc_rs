//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 713/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk713<F: Float>(t1947: F, t8426: F, t473: F, t1510: F, t493: F, t2928: F, t1273: F, t991: F, t1007: F, t1484: F, t1492: F, t433: F, t463: F) -> (F, F, F, F, F, F) {
    let t8427 = t8426 * t1947;
    let t8428 = t473 * t8427;
    let t8430 = t493 * t1510;
    let t8431 = t2928 * t8430;
    let t8433 = t1273 * t991;
    let t8435 = t1484 * t1007;
    let t8437 = t1492 * t1007;
    let t8442 = t463 * t433;
    (t8428, t8431, t8433, t8435, t8437, t8442)
}
