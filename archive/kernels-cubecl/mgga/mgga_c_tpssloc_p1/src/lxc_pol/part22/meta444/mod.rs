//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta444 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1794;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1795;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta444<F: Float>(t16132: F, t1825: F, t1352: F, t19743: F, t19660: F, t118: F, t6330: F, t794: F, t12202: F, t19631: F, t210: F, t214: F, t6347: F, t3739: F, t12211: F, t6353: F, t213: F, t1307: F, t221: F, t5187: F, t5196: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t19756, t19761, t19763, t19767, t19768, t19771) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1794::<F>(t16132, t1825, t1352, t19743, t19660, t118, t6330, t794, t12202, t19631, t210, t214);
        let (t19775, t19776, t19779, t19781, t19783, t19787) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1795::<F>(t118, t6347, t794, t3739, t12211, t6353, t213, t6330, t1307, t221, t5187, t5196);
    (t19756, t19761, t19763, t19767, t19768, t19771, t19775, t19776, t19779, t19781, t19783, t19787)
}
