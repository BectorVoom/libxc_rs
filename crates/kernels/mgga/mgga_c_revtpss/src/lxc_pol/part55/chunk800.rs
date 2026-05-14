//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 800/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk800<F: Float>(t1256: F, t7610: F, t2138: F, t3666: F, t3678: F, t7613: F, t3685: F, t7607: F, t1230: F, t7623: F, t3636: F, t7624: F, t3704: F, t7618: F, t479: F, t3089: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26822 = t7610 * t1256;
    let t26827 = t3666 * t2138;
    let t26832 = t7613 * t3678;
    let t26836 = t7607 * t3685;
    let t26852 = t1230 * t7623;
    let t26855 = t7624 * t3636;
    let t26863 = t7618 * t3704;
    let t26865 = sigma2 * t479;
    let t26866 = t26865 * t3089;
    (t26822, t26827, t26832, t26836, t26852, t26855, t26863, t26865, t26866)
}
