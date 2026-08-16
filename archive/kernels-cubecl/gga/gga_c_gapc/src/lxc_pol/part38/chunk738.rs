//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 738/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk738<F: Float>(t205: F, t8690: F, t1587: F, t1720: F, t3104: F, t1707: F, t3103: F, t1504: F, t126: F, t417: F, t581: F, t3105: F) -> (F, F, F, F, F) {
    let t8691 = t8690 * t205;
    let t8693 = t1720 * t1587;
    let t8694 = t3104 * t8693;
    let t8696 = t1707 * t3103;
    let t8697 = t1720 * t1504;
    let t8698 = t8696 * t8697;
    let t8700 = t126 * t417;
    let t8701 = t581 * t8700;
    let t8702 = t8701 * t3105;
    (t8691, t8694, t8698, t8700, t8702)
}
