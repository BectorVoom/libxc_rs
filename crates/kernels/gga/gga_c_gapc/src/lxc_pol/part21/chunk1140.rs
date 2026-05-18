//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1140/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1140<F: Float>(t33560: F, t9419: F, t11808: F, t29516: F, t3707: F, t4780: F, t2660: F, t7375: F, t33312: F, t3789: F, t11449: F, t11804: F, t15843: F, t190: F, t2674: F) -> (F, F, F, F, F, F) {
    let t34119 = t33560 * t9419;
    let t34121 = t11808 * t29516;
    let t34123 = t4780 * t3707;
    let t34125 = t2660 * t34123 * t7375;
    let t34127 = t33312 * t3789;
    let t34132 = t2674 * t190 * t11449 * t11804 * t15843;
    (t34119, t34121, t34123, t34125, t34127, t34132)
}
