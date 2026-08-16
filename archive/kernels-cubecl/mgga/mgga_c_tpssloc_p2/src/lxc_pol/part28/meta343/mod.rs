//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta343 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1296;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1297;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1298;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1299;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1300;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta343<F: Float>(t40: F, t1409: F, t9427: F, t2433: F, t3966: F, t12606: F, t2244: F, t2250: F, t4080: F, t607: F, t73: F, t9438: F, t2440: F, zeta_threshold: F, t52: F, t4087: F, t76: F, t157: F, t182: F, t145: F, t185: F, t4195: F, t4194: F, t4303: F, t870: F, t262: F, t4119: F, t2553: F, t4315: F, t9717: F, t12850: F, t12854: F, t12860: F, t12861: F, t1877: F, t2522: F, t4310: F, t4314: F, t776: F, t868: F, t9457: F, t9462: F, t9469: F, t9476: F, t9484: F, t9496: F, t9715: F, t9929: F, t4196: F, t9726: F, t10143: F, t1530: F, t2430: F, t4205: F, t750: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t12873, t12874, t12877) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1296::<F>(t40, t1409, t9427, t2433, t3966, t12606, t2244, t2250, t4080, t607, t73, t9438, t2440, zeta_threshold);
        let (t12889, t12890) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1297::<F>(t52, t12606, t12874, t12877, t2244, t2250, t4087, t607, t76, t12873, t157, t182, t145, zeta_threshold);
        let (t12891, t12894, t12895, t12899, t12903, t12906) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1298::<F>(t12890, t185, t2250, t4195, t4194, t4303, t870, t262, t4119, t2553, t4315, t9717);
        let t12907 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1299::<F>(t12850, t12854, t12860, t12861, t12889, t12891, t12894, t12895, t12899, t12903, t12906, t1877, t2522, t2553, t4310, t4314, t776, t868, t9457, t9462, t9469, t9476, t9484, t9496, t9715);
        let (t12910, t12914, t12915, t12922, t12926) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1300::<F>(t157, t9929, t4196, t9726, t10143, t1530, t2430, t4205, t1409, t750, t607, t4194);
    (t12889, t12891, t12894, t12906, t12907, t12910, t12914, t12915, t12922, t12926)
}
