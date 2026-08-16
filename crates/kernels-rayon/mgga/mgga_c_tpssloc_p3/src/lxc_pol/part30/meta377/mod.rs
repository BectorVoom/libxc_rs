//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta377 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1434;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1435;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1436;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta377(t16562: f64, t16574: f64, t145: f64, t185: f64, t5520: f64, t751: f64, t157: f64, t182: f64, t12861: f64, t4119: f64, t4315: f64, t5392: f64, t2658: f64, t2523: f64, t5527: f64, t262: f64, t5544: f64, t1484: f64, t868: f64, t5660: f64, t870: f64, t12850: f64, t12860: f64, t2522: f64, t4307: f64, t4310: f64, t4314: f64, t776: f64, t9457: f64, t9469: f64, t9476: f64, t9484: f64, t9496: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t16577, t16578, t16581, t16582, t16583, t16586) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1434(t16562, t16574, t145, t185, t5520, t751, t157, t182, t12861, t4119, t4315, t5392);
        let (t16588, t16589, t16592, t16596) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1435(t16586, t2658, t2523, t5527, t262, t5544, t1484, t868);
        let t16610 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1436(t5660, t870, t12850, t12860, t16577, t16578, t16581, t16582, t16583, t16588, t16589, t16592, t16596, t2522, t2523, t4119, t4307, t4310, t4314, t5544, t776, t9457, t9469, t9476, t9484, t9496);
    (t16577, t16578, t16581, t16582, t16588, t16596, t16610)
}
