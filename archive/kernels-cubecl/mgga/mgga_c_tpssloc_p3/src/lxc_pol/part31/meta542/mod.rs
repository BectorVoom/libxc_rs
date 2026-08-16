//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta542 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1763;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1764;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta542<F: Float>(t6892: F, t81186: F, t1987: F, t81144: F, t9537: F, t107: F, t835: F, t240: F, t656: F, t666: F, t2331: F, t625: F, t63: F, t9365: F, t2752: F, t606: F, t22641: F, t9523: F, t22690: F, t6639: F, t1887: F, t23069: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t81375, t81398, t81437, t81439, t81440, t81442) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1763::<F>(t6892, t81186, t1987, t81144, t9537, t107, t835, t240, t656, t666, t2331, t625);
        let (t81446, t81547, t81573, t81575, t81591) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1764::<F>(t63, t9365, t2752, t606, t22641, t9523, t22690, t6639, t1887, t23069);
    (t81375, t81398, t81437, t81439, t81440, t81442, t81446, t81547, t81573, t81575, t81591)
}
