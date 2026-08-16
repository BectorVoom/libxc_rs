//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1151/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1151<F: Float>(t11417: F, t128: F, t7333: F, t932: F, t935: F, t11733: F, t949: F, t1971: F, t9066: F, t2660: F, t8135: F, t11905: F, t18815: F) -> (F, F, F, F, F, F) {
    let t33369 = t932 * t11417 * t7333 * t935 * t128;
    let t33371 = t11733 * t949;
    let t33373 = t1971 * t9066;
    let t33374 = t2660 * t33373;
    let t33375 = t33374 * t8135;
    let t33377 = t11905 * t18815;
    (t33369, t33371, t33373, t33374, t33375, t33377)
}
