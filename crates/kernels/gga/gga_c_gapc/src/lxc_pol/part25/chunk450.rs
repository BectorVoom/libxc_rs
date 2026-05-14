//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 450/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk450<F: Float>(t1647: F, t2580: F, t597: F, t818: F, t906: F, t871: F, t897: F, t1686: F, t933: F, t786: F, t1086: F, t1087: F, t103: F, t327: F) -> (F, F, F, F, F, F) {
    let t2581 = t1647 * t2580;
    let t2585 = t597 * t818 * t906;
    let t2588 = t871 * t897;
    let t2591 = t933 * t1686;
    let t2592 = t786 * t818;
    let t2594 = t1086 * t1087 * t2592;
    let t2597 = t103 * t327;
    (t2581, t2585, t2588, t2591, t2594, t2597)
}
