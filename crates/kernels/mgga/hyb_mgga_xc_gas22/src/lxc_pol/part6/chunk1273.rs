//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1273/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1273<F: Float>(t29757: F, t29760: F, t29788: F, t29822: F, t29825: F, t29827: F, t29833: F, t29836: F, t29839: F, t29842: F, t29844: F, t29846: F, t10914: F, t2484: F, t952: F, t10892: F, t2490: F) -> (F, F, F) {
    let t29848 = 0.16504875e0 * t29822 - 0.258925e1 * t29825 + 0.16504875e0 * t29827 + 0.40256666666666666667e0 * t29757 - 0.60385e0 * t29760 + 0.905775e0 * t29788 - 0.485484375e1 * t29833 + 0.19419375e1 * t29836 + 0.6189328125e-1 * t29839 - 0.412621875e-1 * t29842 + 0.19419375e1 * t29844 - 0.258925e1 * t29846;
    let t29851 = t2484 * t10914 * t952;
    let t29853 = t10892 * t2490;
    (t29848, t29851, t29853)
}
