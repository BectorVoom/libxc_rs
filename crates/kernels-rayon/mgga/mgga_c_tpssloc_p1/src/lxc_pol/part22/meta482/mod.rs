//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta482 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1889;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1890;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1891;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1892;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1893;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta482(t4483: f64, t5812: f64, t1568: f64, t5742: f64, t2888: f64, t10277: f64, t20234: f64, t2826: f64, t136: f64, t4337: f64, t5398: f64, t2768: f64, t123: f64, t4342: f64, t882: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21107, t21114, t21115, t21118) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1889(t4483, t5812, t1568, t5742, t2888, t10277, t20234);
        let (t21119, t21120, t21122) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1890(t21118, t2826, t136, t4337, t5398);
        let (t21123, t21124) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1891(t21122, t2768, t123);
        let t21126 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1892(t4342, t5398);
        let (t21127, t21128) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1893(t21126, t882, t123);
    (t21107, t21114, t21115, t21118, t21119, t21120, t21122, t21123, t21124, t21126, t21127, t21128)
}
