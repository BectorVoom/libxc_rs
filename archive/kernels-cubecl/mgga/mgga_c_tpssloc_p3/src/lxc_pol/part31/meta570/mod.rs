//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta570 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1802;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1803;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta570<F: Float>(t25064: F, t81902: F, t23077: F, t6646: F, t6590: F, t23033: F, t25155: F, t6546: F, t25112: F, t81835: F, t23083: F, t25116: F, t22996: F, t23110: F, t25299: F, t81651: F, t23168: F, t25313: F, t252: F, t87230: F, t25321: F, t25284: F, t6579: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t87445, t87447, t87451, t87463, t87477, t87487) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1802::<F>(t25064, t81902, t23077, t6646, t6590, t23033, t25155, t6546, t25112, t81835, t23083, t25116);
        let (t87504, t87520, t87522, t87529, t87533, t87535) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1803::<F>(t22996, t6590, t23110, t25299, t81651, t23168, t25313, t252, t87230, t25321, t25284, t6579);
    (t87445, t87447, t87451, t87463, t87477, t87487, t87504, t87520, t87522, t87529, t87533, t87535)
}
