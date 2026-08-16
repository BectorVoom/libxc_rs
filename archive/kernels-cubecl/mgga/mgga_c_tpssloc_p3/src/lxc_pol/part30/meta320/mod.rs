//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta320 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1344;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta320<F: Float>(t3242: F, t460: F, t3247: F, t1176: F, t134: F, t1184: F, t1239: F, t68: F, t1203: F, t3540: F, t2393: F, t374: F, t486: F) -> (F, F, F, F, F, F, F) {
        let (t11570, t11583, t11588, t11589, t11606, t11644, t11647) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1344::<F>(t3242, t460, t3247, t1176, t134, t1184, t1239, t68, t1203, t3540, t2393, t374, t486);
    (t11570, t11583, t11588, t11589, t11606, t11644, t11647)
}
