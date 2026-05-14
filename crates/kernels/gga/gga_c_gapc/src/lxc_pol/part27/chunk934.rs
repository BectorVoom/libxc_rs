//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 934/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk934<F: Float>(t23104: F, t8676: F, t10078: F, t2763: F, t818: F, t941: F, t103: F, t134: F, t18679: F, t15479: F, t2547: F, t9933: F, t126: F, t932: F, t1038: F, t11925: F, t16826: F, t19: F, t7877: F) -> (F, F, F, F, F, F, F) {
    let t29692 = t8676 * t23104;
    let t29861 = t818 * t2763 * t941 * t10078;
    let t29867 = t134 * t18679 * t103;
    let t29868 = t15479 * t941 * t29867;
    let t30095 = t2547 * t9933;
    let t30153 = t932 * t126;
    let t30158 = t11925 * t1038 * t7877 * t19 * t16826;
    (t29692, t29861, t29867, t29868, t30095, t30153, t30158)
}
