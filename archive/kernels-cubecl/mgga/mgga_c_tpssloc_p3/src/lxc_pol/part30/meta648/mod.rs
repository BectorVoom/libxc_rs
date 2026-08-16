//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta648 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2062;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta648<F: Float>(t23384: F, t25811: F, t25407: F, t25513: F, t82431: F, t25726: F, t25608: F, t6743: F, t23631: F, t61066: F, t974: F, t23665: F, t25524: F) -> (F, F, F, F, F, F, F) {
        let (t88937, t88954, t88992, t88998, t89002, t89033, t89049) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2062::<F>(t23384, t25811, t25407, t25513, t82431, t25726, t25608, t6743, t23631, t61066, t974, t23665, t25524);
    (t88937, t88954, t88992, t88998, t89002, t89033, t89049)
}
