//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta148 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk696;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk697;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk698;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta148(t1088: f64, t5975: f64, t123: f64, t1089: f64, t5398: f64, t3237: f64, t4721: f64, t5973: f64, t423: f64, t1671: f64, t4740: f64, t1670: f64, t1118: f64, t3264: f64, t1661: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5976, t5977, t5979) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk696(t1088, t5975, t123, t1089, t5398);
        let (t5980, t5981, t5983, t5985, t5987, t5988) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk697(t1088, t5979, t123, t3237, t4721, t5973, t5977, t423, t1671, t4740, t1670);
        let (t5989, t5991, t5992) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk698(t1118, t5988, t3264, t1661);
    (t5976, t5977, t5979, t5980, t5981, t5983, t5985, t5987, t5988, t5989, t5991, t5992)
}
