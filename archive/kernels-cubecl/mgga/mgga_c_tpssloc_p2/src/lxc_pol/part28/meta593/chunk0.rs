//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1889/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1889<F: Float>(t1081: F, t4255: F, t870: F, t23788: F, t58071: F, t86706: F, t1649: F, t2745: F, t25927: F, t86713: F, t2379: F, t2553: F) -> (F, F, F, F, F, F, F) {
    let t89859 = t870 * t1081 * t4255;
    let t89862 = t23788 * t58071;
    let t89865 = t23788 * t86706;
    let t89868 = t1649 * t2745;
    let t89872 = t25927 * t86713;
    let t89874 = t1649 * t2379;
    let t89881 = t1649 * t2553;
    (t89859, t89862, t89865, t89868, t89872, t89874, t89881)
}
