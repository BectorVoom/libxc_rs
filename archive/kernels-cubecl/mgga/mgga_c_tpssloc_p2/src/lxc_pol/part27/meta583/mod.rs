//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta583 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2035;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2036;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta583<F: Float>(t22705: F, t22733: F, t81228: F, t22724: F, t22927: F, t22642: F, t22643: F, t6907: F, t22644: F, t81152: F, t6891: F, t81195: F, t22649: F, t6883: F, t1372: F, t212: F, t6890: F, t1988: F, t81071: F, t225: F, t22942: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t81230, t81264, t81267, t81282, t81284) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2035::<F>(t22705, t22733, t81228, t22724, t22927, t22642, t22643, t6907, t22644, t81152, t6891, t81195);
        let (t81307, t81311, t81318, t81319, t81326) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2036::<F>(t22649, t6883, t1372, t212, t22642, t6890, t1988, t81071, t225, t22942, t22643);
    (t81230, t81264, t81267, t81282, t81284, t81307, t81311, t81318, t81319, t81326)
}
