//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 604/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk604<F: Float>(t1438: F, t428: F, t4688: F, t4711: F, t4714: F, t4718: F, t4811: F, t4815: F, t4818: F, t4820: F, t4822: F, t4824: F, t4826: F, t1333: F, t4778: F, t87: F) -> (F, F, F, F, F, F) {
    let t4827 = t1438 * t428;
    let t4828 = 96.0 * t4827;
    let t4829 = t4811 - t4815 + t4688 + t4711 - t4714 - t4718 - t4818 + t4820 - t4822 + t4824 + t4826 - t4828;
    let t4830 = t1333 * t428;
    let t4831 = 60.0 * t4830;
    let t4832 = t4778 * t87;
    (t4827, t4828, t4829, t4830, t4831, t4832)
}
