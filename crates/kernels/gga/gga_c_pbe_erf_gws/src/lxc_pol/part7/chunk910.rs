//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 910/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk910<F: Float>(t17672: F, t17771: F, t17773: F, t17778: F, t17783: F, t17785: F, t17790: F, t17794: F, t17796: F, t17798: F, t17800: F, t17803: F, t17807: F, t17809: F, t17813: F, t17815: F, t17818: F, t17822: F, t17825: F, t17827: F, t17832: F, t17836: F, t17838: F) -> (F, F) {
    let t18325 = -t17672 - t17771 - t17773 - t17778 - t17783 + t17785 + t17790 + t17794 - t17796 - t17798 + t17800;
    let t18326 = -t17803 - t17807 + t17809 + t17813 - t17815 + t17818 - t17822 - t17825 - t17827 + t17832 + t17836 + t17838;
    (t18325, t18326)
}
