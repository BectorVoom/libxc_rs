//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta562 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1791;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1792;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta562<F: Float>(t81575: F, t25251: F, t87049: F, t23012: F, t7529: F, t23110: F, t23185: F, t25241: F, t1484: F, t852: F, t81595: F, t81602: F, t252: F, t4119: F, t22690: F, t7520: F, t81573: F, t25324: F, t6562: F, t794: F, t23030: F, t25258: F, t22893: F, t23164: F, t25306: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t87073, t87078, t87080, t87100, t87111, t87119, t87127) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1791::<F>(t81575, t25251, t87049, t23012, t7529, t23110, t23185, t25241, t1484, t852, t81595, t81602);
        let (t87130, t87140, t87153, t87155, t87165) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1792::<F>(t252, t4119, t22690, t7520, t81573, t25324, t6562, t794, t23030, t25258, t22893, t23164, t25306);
    (t87073, t87078, t87080, t87100, t87111, t87119, t87127, t87130, t87140, t87153, t87155, t87165)
}
