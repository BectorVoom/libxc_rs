//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta639 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2178;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2179;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta639<F: Float>(t40018: F, t6353: F, t12189: F, t6358: F, t16081: F, t19795: F, t1307: F, t54718: F, t56463: F, t686: F, t16094: F, t16095: F, t5187: F, t56467: F, t19767: F, t40409: F, t19771: F, t3726: F, t12199: F, t19775: F, t19783: F, t54670: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t56484, t56491, t56493, t56501, t56505) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2178::<F>(t40018, t6353, t12189, t6358, t16081, t19795, t1307, t54718, t56463, t686, t16094, t16095, t5187);
        let (t56514, t56535, t56537, t56539, t56548) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2179::<F>(t1307, t16094, t56467, t686, t19767, t40409, t19771, t3726, t12199, t19775, t19783, t54670);
    (t56484, t56491, t56493, t56501, t56505, t56514, t56535, t56537, t56539, t56548)
}
