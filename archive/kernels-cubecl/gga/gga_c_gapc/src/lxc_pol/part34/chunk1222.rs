//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1222/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1222<F: Float>(t11325: F, t3060: F, t8621: F, t185: F, t33643: F, t11489: F, t1038: F, t152: F, t1875: F, t33722: F, t5918: F, t20774: F, t26312: F, t2993: F) -> (F, F, F, F, F, F) {
    let t34465 = t3060 * t11325;
    let t34466 = t34465 * t8621;
    let t34468 = t185 * t33643;
    let t34469 = t34468 * t11489;
    let t34474 = t1875 * t33722 * t1038 * t152 * t5918;
    let t34477 = t2993 * t26312 * t20774;
    (t34465, t34466, t34468, t34469, t34474, t34477)
}
