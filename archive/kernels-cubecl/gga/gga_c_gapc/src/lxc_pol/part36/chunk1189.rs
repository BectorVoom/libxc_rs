//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1189/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1189<F: Float>(t116: F, t1899: F, t33666: F, t34819: F, t11503: F, t8916: F, t34793: F, t8684: F, t11526: F, t26609: F, t129: F, t21655: F, t21657: F, t3021: F) -> (F, F, F, F, F) {
    let t34820 = t116 * t1899 * t33666 * t34819;
    let t34822 = t8916 * t11503;
    let t34824 = t8684 * t34793;
    let t34826 = t11526 * t26609;
    let t34830 = t21655 * t129 * t3021 * t21657;
    (t34820, t34822, t34824, t34826, t34830)
}
