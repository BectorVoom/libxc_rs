//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta476 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1846;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1847;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta476(t23445: f64, t23486: f64, t23532: f64, t23569: f64, t349: f64, t23346: f64, t23385: f64, t23387: f64, t23389: f64, t23392: f64, t23396: f64, t23399: f64, t23403: f64, t23408: f64, t23410: f64, t388: f64, t6687: f64, t6692: f64, t23384: f64, t1049: f64, t6688: f64, t6691: f64, t1054: f64, t1065: f64, t1921: f64, t986: f64, t2978: f64, t344: f64, t381: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23571, t23572, t23574) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1846(t23445, t23486, t23532, t23569, t349, t23346, t23385, t23387, t23389, t23392, t23396, t23399, t23403, t23408, t23410, t388, t6687, t6692);
        let (t23579, t23581, t23582, t23587, t23588, t23589, t23592, t23593) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1847(t23384, t6692, t1049, t6688, t6691, t1054, t1065, t1921, t986, t2978, t344, t381);
    (t23571, t23572, t23574, t23579, t23581, t23582, t23587, t23588, t23589, t23592, t23593)
}
