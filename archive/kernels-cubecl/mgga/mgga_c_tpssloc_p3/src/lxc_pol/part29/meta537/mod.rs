//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta537 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1925;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta537<F: Float>(t26384: F, t6637: F, t6888: F, t5187: F, t6968: F, t22893: F, t7732: F, t22892: F, t1834: F, t552: F, t1307: F, t26328: F, t553: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t26385, t26386, t26388, t26389, t26390, t26392, t26393, t26395, t26396, t26397, t26398, t26401) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1925::<F>(t26384, t6637, t6888, t5187, t6968, t22893, t7732, t22892, t1834, t552, t1307, t26328, t553);
    (t26385, t26386, t26388, t26389, t26390, t26392, t26393, t26395, t26396, t26397, t26398, t26401)
}
