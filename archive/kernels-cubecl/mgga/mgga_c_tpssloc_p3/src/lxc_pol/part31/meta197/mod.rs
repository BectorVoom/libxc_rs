//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta197 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk875;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk876;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk877;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta197<F: Float>(t3805: F, t3807: F, t5249: F, t2408: F, t2417: F, t2423: F, t3686: F, t3688: F, t3690: F, t3695: F, t3813: F, t5153: F, t5156: F, t5159: F, t5164: F, t5167: F, t3815: F, t1788: F, t588: F, t592: F, t3829: F, t3833: F, t2426: F, t2486: F, t3819: F, t3821: F, t3825: F, t3827: F, t3832: F, t5169: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t5259 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk875::<F>(t3805, t3807, t5249);
        let t5262 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk876::<F>(t2408, t2417, t2423, t3686, t3688, t3690, t3695, t3813, t5153, t5156, t5159, t5164, t5167);
        let (t5263, t5264, t5265, t5266, t5267, t5268, t5269, t5270) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk877::<F>(t3815, t1788, t588, t592, t3829, t3833, t2426, t2486, t3819, t3821, t3825, t3827, t3832, t5169);
    (t5259, t5262, t5263, t5264, t5265, t5266, t5267, t5268, t5269, t5270)
}
