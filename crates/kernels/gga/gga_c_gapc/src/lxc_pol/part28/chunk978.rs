//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 978/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk978<F: Float>(t2999: F, t5216: F, t1648: F, t3005: F, t154: F, t3949: F, t126: F, t632: F, t1038: F, t11589: F, t147: F, t19509: F, t457: F, t137: F, t27144: F, t1552: F, t3143: F, t674: F) -> (F, F, F, F, F, F, F) {
    let t27867 = t2999 * t5216;
    let t27868 = t1648 * t3005 * t27867;
    let t27889 = t154 * t3949;
    let t27935 = t632 * t126;
    let t27940 = t11589 * t1038 * t19509 * t147 * t457;
    let t28006 = t27144 * t137;
    let t28065 = M_PI * t1552 * t674 * t3143;
    (t27867, t27868, t27889, t27935, t27940, t28006, t28065)
}
