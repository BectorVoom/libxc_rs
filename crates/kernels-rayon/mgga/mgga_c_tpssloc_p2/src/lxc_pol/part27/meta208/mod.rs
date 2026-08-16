//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta208 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1039;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1040;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1041;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1042;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1043;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1044;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta208(t1065: f64, t1634: f64, t3174: f64, t1057: f64, t4639: f64, t1022: f64, t3188: f64, t1629: f64, t1049: f64, t1615: f64, t1060: f64, t381: f64, t4649: f64, t1932: f64, t360: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4664, t4665) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1039(t1065, t1634, t3174);
        let t4669 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1040(t1057, t4639);
        let t4673 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1041(t1022, t3188);
        let (t4674, t4677) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1042(t1629, t4673, t1049, t1615);
        let (t4678, t4680) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1043(t1060, t4677, t381, t4649);
        let (t4681, t4684) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1044(t1060, t4680, t1022, t1932, t360);
    (t4664, t4665, t4669, t4673, t4674, t4677, t4678, t4680, t4681, t4684)
}
