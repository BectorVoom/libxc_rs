//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1057/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1057<F: Float>(t11320: F, t11322: F, t1932: F, t11321: F, t4925: F, t8950: F, t11508: F, t1749: F, t3060: F, t11325: F, t8621: F, t185: F, t33643: F, t11489: F, t1038: F, t152: F, t1875: F, t33722: F, t5918: F) -> (F, F, F, F, F, F, F, F) {
    let t34457 = t1932 * t11320 * t11322;
    let t34460 = t11321 * t4925 * t8950;
    let t34463 = t3060 * t11508 * t1749;
    let t34465 = t3060 * t11325;
    let t34466 = t34465 * t8621;
    let t34468 = t185 * t33643;
    let t34469 = t34468 * t11489;
    let t34474 = t1875 * t33722 * t1038 * t152 * t5918;
    (t34457, t34460, t34463, t34465, t34466, t34468, t34469, t34474)
}
