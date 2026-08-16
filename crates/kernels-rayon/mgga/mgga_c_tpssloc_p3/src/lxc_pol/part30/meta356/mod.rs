//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta356 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1396;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1397;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta356(t1603: f64, t3030: f64, t3032: f64, t3129: f64, t3038: f64, t225: f64, t4658: f64, t4553: f64, t4559: f64, t4555: f64, t3199: f64, t3185: f64, t1057: f64, t14205: f64, t1654: f64, t2394: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14507, t14508, t14511, t14529, t14545, t14552, t14555, t14608, t14618) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1396(t1603, t3030, t3032, t3129, t3038, t225, t4658, t4553, t4559, t4555, t3199, t3185);
        let (t14651, t14702) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1397(t1057, t14205, t1654, t2394);
    (t14507, t14508, t14511, t14529, t14545, t14552, t14555, t14608, t14618, t14651, t14702)
}
