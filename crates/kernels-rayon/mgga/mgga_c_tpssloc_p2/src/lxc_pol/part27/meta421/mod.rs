//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta421 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1729;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1730;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta421(t15904: f64, t8643: f64, t22574: f64, t3701: f64, t3914: f64, t2019: f64, t1983: f64, t6996: f64, t6999: f64, t1390: f64, t3719: f64, t6878: f64, t1266: f64, t1393: f64, t1869: f64, t1976: f64, t1980: f64, t22460: f64, t22461: f64, t22467: f64, t22482: f64, t22483: f64, t22559: f64, t22563: f64, t2314: f64, t2320: f64, t2323: f64, t3652: f64, t3929: f64, t510: f64, t650: f64, t6515: f64, t6517: f64, t652: f64, t6539: f64, t672: f64, t6862: f64, t6872: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t22575, t22577, t22578, t22579, t22580, t22581, t22583, t22584, t22585) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1729(t15904, t8643, t22574, t3701, t3914, t2019, t1983, t6996, t6999, t1390, t3719, t6878);
        let t22588 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1730(t1983, t22585, t1266, t1393, t1869, t1976, t1980, t22460, t22461, t22467, t22482, t22483, t22559, t22563, t22577, t22580, t22583, t2314, t2320, t2323, t3652, t3929, t510, t650, t6515, t6517, t652, t6539, t672, t6862, t6872);
    (t22575, t22578, t22579, t22581, t22584, t22585, t22588)
}
