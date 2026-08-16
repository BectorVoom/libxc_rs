//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta343 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1296;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1297;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1298;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1299;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1300;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta343(t40: f64, t1409: f64, t9427: f64, t2433: f64, t3966: f64, t12606: f64, t2244: f64, t2250: f64, t4080: f64, t607: f64, t73: f64, t9438: f64, t2440: f64, zeta_threshold: f64, t52: f64, t4087: f64, t76: f64, t157: f64, t182: f64, t145: f64, t185: f64, t4195: f64, t4194: f64, t4303: f64, t870: f64, t262: f64, t4119: f64, t2553: f64, t4315: f64, t9717: f64, t12850: f64, t12854: f64, t12860: f64, t12861: f64, t1877: f64, t2522: f64, t4310: f64, t4314: f64, t776: f64, t868: f64, t9457: f64, t9462: f64, t9469: f64, t9476: f64, t9484: f64, t9496: f64, t9715: f64, t9929: f64, t4196: f64, t9726: f64, t10143: f64, t1530: f64, t2430: f64, t4205: f64, t750: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12873, t12874, t12877) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1296(t40, t1409, t9427, t2433, t3966, t12606, t2244, t2250, t4080, t607, t73, t9438, t2440, zeta_threshold);
        let (t12889, t12890) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1297(t52, t12606, t12874, t12877, t2244, t2250, t4087, t607, t76, t12873, t157, t182, t145, zeta_threshold);
        let (t12891, t12894, t12895, t12899, t12903, t12906) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1298(t12890, t185, t2250, t4195, t4194, t4303, t870, t262, t4119, t2553, t4315, t9717);
        let t12907 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1299(t12850, t12854, t12860, t12861, t12889, t12891, t12894, t12895, t12899, t12903, t12906, t1877, t2522, t2553, t4310, t4314, t776, t868, t9457, t9462, t9469, t9476, t9484, t9496, t9715);
        let (t12910, t12914, t12915, t12922, t12926) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1300(t157, t9929, t4196, t9726, t10143, t1530, t2430, t4205, t1409, t750, t607, t4194);
    (t12889, t12891, t12894, t12906, t12907, t12910, t12914, t12915, t12922, t12926)
}
