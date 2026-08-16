//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta322 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1346;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1347;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta322<F: Float>(t11707: F, t3032: F, t3505: F, t10469: F, t466: F, t10471: F, t1208: F, t478: F, t10477: F, t483: F, t3508: F, t475: F, t3503: F, t3514: F, t1210: F, t3247: F, t415: F, t121: F, t3584: F, t1229: F, t676: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t11708, t11709, t11712, t11713, t11715, t11717, t11719, t11721) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1346::<F>(t11707, t3032, t3505, t10469, t466, t10471, t1208, t478, t10477, t483, t3508, t475);
        let (t11728, t11734, t11738, t11778, t11784, t11789) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1347::<F>(t11717, t3503, t11713, t11708, t3514, t1210, t3247, t415, t121, t3584, t1229, t676);
    (t11709, t11712, t11715, t11719, t11721, t11728, t11734, t11738, t11778, t11784, t11789)
}
