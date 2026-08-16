//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta483 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1859;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta483<F: Float>(t23678: F, t3187: F, t23677: F, t1049: F, t362: F, t884: F, t6784: F, t2780: F, t6785: F, t225: F, t23592: F, t2771: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t23679, t23680, t23685, t23686, t23687, t23692, t23693, t23696, t23697) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1859::<F>(t23678, t3187, t23677, t1049, t362, t884, t6784, t2780, t6785, t225, t23592, t2771);
    (t23679, t23680, t23685, t23686, t23687, t23692, t23693, t23696, t23697)
}
