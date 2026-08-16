//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta209 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk983;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk984;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk985;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk986;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta209(t1484: f64, t40: f64, t52: f64, t5392: f64, t5398: f64, t75: f64, t767: f64, t771: f64, t78: f64, zeta_threshold: f64, t210: f64, t214: f64, t2562: f64, t2569: f64, t2571: f64, t2590: f64, t4124: f64, t4135: f64, t787: f64, t252: f64, t1492: f64, t1519: f64, t119: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t5527 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk983(t1484);
        let t5544 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk984(t40, t52, t5392, t5398, t75, t767, t771, t78, zeta_threshold);
        let (t5550, t5555, t5558) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk985(t210, t214, t5527, t5544, t2562, t2569, t2571, t2590, t4124, t4135, t787);
        let (t5559, t5561, t5567, t5568, t5571, t5572, t5575) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk986(t252, t5558, t1492, t1519, t119, t5527, t210, t5544, t225);
    (t5527, t5544, t5550, t5555, t5558, t5559, t5561, t5567, t5568, t5571, t5572, t5575)
}
