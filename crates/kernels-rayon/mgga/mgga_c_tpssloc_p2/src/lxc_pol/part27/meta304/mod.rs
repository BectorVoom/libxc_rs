//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta304 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1367;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1368;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1369;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1370;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta304(t376: f64, t676: f64, t1023: f64, t248: f64, t1020: f64, t1017: f64, t3087: f64, t1015: f64, t1012: f64, t2928: f64, t320: f64, t10294: f64, t268: f64, t271: f64, t6546: f64, t2394: f64, t885: f64, t2772: f64, t690: f64, t2777: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10508, t10510, t10511, t10515, t10517, t10523, t10542) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1367(t376, t676, t1023, t248, t1020, t1017, t3087, t1015, t1012, t2928, t320, t10294);
        let (t10544, t10545, t10556) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1368(t268, t271, t6546, t2394, t885);
        let t10558 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1369(t2772, t690);
        let t10560 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1370(t2777, t690);
    (t10508, t10510, t10511, t10515, t10517, t10523, t10542, t10544, t10545, t10556, t10558, t10560)
}
