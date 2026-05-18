//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1108/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1108<F: Float>(t1347: F, t7614: F, t2001: F, t5108: F, t1967: F, t8502: F, t4932: F, t4552: F, t1998: F, t5089: F, t1451: F, t7605: F) -> (F, F, F, F, F, F, F) {
    let t35709 = t7614 * t1347;
    let t35720 = t2001 * t5108;
    let t35722 = t1967 * t8502;
    let t35724 = t2001 * t4932;
    let t35731 = t2001 * t4552;
    let t35733 = t1998 * t5089;
    let t35736 = t7605 * t1451;
    (t35709, t35720, t35722, t35724, t35731, t35733, t35736)
}
