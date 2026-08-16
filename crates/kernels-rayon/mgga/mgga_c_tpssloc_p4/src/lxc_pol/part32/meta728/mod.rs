//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta728 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2365;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2366;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2367;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2368;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta728(t100908: f64, t100915: f64, t100917: f64, t100921: f64, t100924: f64, t100927: f64, t100929: f64, t100932: f64, t100934: f64, t100936: f64, t100938: f64, t100941: f64, t1458: f64, t20176: f64, t20181: f64, t24972: f64, t27921: f64, t4072: f64, t5376: f64, t5456: f64, t85416: f64, t96311: f64, t96334: f64, t1851: f64, t8119: f64, t103103: f64, t105102: f64, t105115: f64, t1396: f64, t1398: f64, t1404: f64, t1852: f64, t20149: f64, t2174: f64, t27930: f64, t29866: f64, t29884: f64, t3: f64, t5364: f64, t580: f64, t6483: f64, t7416: f64, t96281: f64, t96283: f64, t96285: f64, t1858: f64, t8110: f64, t29865: f64, t2169: f64, t576: f64, t20186: f64, t2170: f64, t27908: f64, t5381: f64, t6471: f64, t7426: f64, t8111: f64, t96289: f64, t96291: f64, t96300: f64, t96303: f64, t96308: f64) -> f64 {
        let t105128 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2365(t100908, t100915, t100917, t100921, t100924, t100927, t100929, t100932, t100934, t100936, t100938, t100941, t1458, t20176, t20181, t24972, t27921, t4072, t5376, t5456, t85416, t96311, t96334);
        let t105139 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2366(t1851, t8119, t103103, t105102, t105115, t105128, t1396, t1398, t1404, t1852, t20149, t2174, t27930, t29866, t29884, t3, t5364, t580, t6483, t7416, t96281, t96283, t96285);
        let t105151 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2367(t1858, t8110, t29865, t580, t2169, t6483, t29884, t576, t20186, t2170, t27908, t5381, t6471, t7426, t8111, t96289, t96291, t96300, t96303, t96308);
        let tv4rho3sigma8 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2368(t105139, t105151);
    tv4rho3sigma8
}
