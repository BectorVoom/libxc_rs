//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta200 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk942;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk943;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk944;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta200(t3805: f64, t3807: f64, t5249: f64, t2408: f64, t2417: f64, t2423: f64, t3686: f64, t3688: f64, t3690: f64, t3695: f64, t3813: f64, t5153: f64, t5156: f64, t5159: f64, t5164: f64, t5167: f64, t3815: f64, t1788: f64, t588: f64, t592: f64, t3829: f64, t3833: f64, t2426: f64, t2486: f64, t3819: f64, t3821: f64, t3825: f64, t3827: f64, t3832: f64, t5169: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t5259 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk942(t3805, t3807, t5249);
        let t5262 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk943(t2408, t2417, t2423, t3686, t3688, t3690, t3695, t3813, t5153, t5156, t5159, t5164, t5167);
        let (t5263, t5264, t5265, t5266, t5267, t5268, t5269, t5270) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk944(t3815, t1788, t588, t592, t3829, t3833, t2426, t2486, t3819, t3821, t3825, t3827, t3832, t5169);
    (t5259, t5262, t5263, t5264, t5265, t5266, t5267, t5268, t5269, t5270)
}
