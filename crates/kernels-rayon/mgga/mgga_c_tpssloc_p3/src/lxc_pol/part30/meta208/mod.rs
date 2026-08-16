//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta208 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk981;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk982;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta208(t40: f64, t4100: f64, t4102: f64, t185: f64, t5392: f64, t2658: f64, t1484: f64, t4310: f64, t1462: f64, t4205: f64, t2433: f64, t5398: f64, t73: f64, zeta_threshold: f64, t52: f64, t2440: f64, t76: f64, t145: f64, t157: f64, t182: f64, t4200: f64, t2373: f64, t2377: f64, t2408: f64, t2417: f64, t2522: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5497, t5498, t5499, t5501, t5502, t5506, t5512) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk981(t40, t4100, t4102, t185, t5392, t2658, t1484, t4310, t1462, t4205, t2433, t5398, t73, zeta_threshold);
        let (t5519, t5520, t5521, t5522, t5524, t5525, t5526) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk982(t52, t2440, t5392, t5398, t76, t5512, t145, t185, t157, t182, t4200, t2373, t2377, t2408, t2417, t2522, t5497, t5498, t5501, t5502, t5506, zeta_threshold);
    (t5497, t5498, t5499, t5501, t5506, t5519, t5520, t5521, t5522, t5524, t5525, t5526)
}
