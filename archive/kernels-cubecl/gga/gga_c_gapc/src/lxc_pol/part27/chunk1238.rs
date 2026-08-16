//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1238/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1238<F: Float>(t1030: F, t33152: F, t9262: F, t11357: F, t26102: F, t11588: F, t27043: F, t35175: F, t3703: F, t11418: F, t3141: F, t34863: F, t505: F) -> (F, F, F, F, F) {
    let t35283 = t1030 * t33152 * t9262;
    let t35285 = t11357 * t26102;
    let t35287 = t11588 * t27043;
    let t35289 = t35175 * t3703;
    let t35293 = t11418 * t3141 * t34863 * t505;
    (t35283, t35285, t35287, t35289, t35293)
}
