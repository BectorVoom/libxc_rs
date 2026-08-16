//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta639 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2178;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2179;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta639(t40018: f64, t6353: f64, t12189: f64, t6358: f64, t16081: f64, t19795: f64, t1307: f64, t54718: f64, t56463: f64, t686: f64, t16094: f64, t16095: f64, t5187: f64, t56467: f64, t19767: f64, t40409: f64, t19771: f64, t3726: f64, t12199: f64, t19775: f64, t19783: f64, t54670: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t56484, t56491, t56493, t56501, t56505) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2178(t40018, t6353, t12189, t6358, t16081, t19795, t1307, t54718, t56463, t686, t16094, t16095, t5187);
        let (t56514, t56535, t56537, t56539, t56548) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2179(t1307, t16094, t56467, t686, t19767, t40409, t19771, t3726, t12199, t19775, t19783, t54670);
    (t56484, t56491, t56493, t56501, t56505, t56514, t56535, t56537, t56539, t56548)
}
