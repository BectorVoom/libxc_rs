//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta435 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1732;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1733;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1734;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1735;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta435<F: Float>(t22705: F, t6978: F, t22704: F, t2006: F, t3787: F, t3793: F, t154: F, t2558: F, t1984: F, t2010: F, t1998: F, t3879: F, t214: F, t1985: F, t591: F, t6896: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t22706, t22707, t22710, t22715) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1732::<F>(t22705, t6978, t22704, t2006, t3787, t3793, t154, t2558);
        let t22716 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1733::<F>(t1984, t22715);
        let (t22718, t22719, t22720, t22721, t22723) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1734::<F>(t2010, t22716, t1998, t3879, t214, t1985, t154, t591);
        let t22724 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1735::<F>(t22723, t6896);
    (t22706, t22707, t22710, t22715, t22716, t22718, t22719, t22720, t22721, t22723, t22724)
}
