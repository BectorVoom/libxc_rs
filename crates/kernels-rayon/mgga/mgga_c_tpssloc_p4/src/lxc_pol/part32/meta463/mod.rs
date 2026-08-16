//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta463 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1747;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1748;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1749;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1750;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta463(t225: f64, t6625: f64, t6576: f64, t2752: f64, t6665: f64, t10143: f64, t1914: f64, t134: f64, t221: f64, t3034: f64, t371: f64, t28: f64, t2274: f64, t50: f64, t7245: f64, t9239: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23278, t23281, t23290) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1747(t225, t6625, t6576, t2752, t6665);
        let t23295 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1748(t10143, t1914);
        let (t23383, t23508, t23598, t23788) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1749(t134, t221, t3034, t371, t2752, t28);
        let (t24498, t24514) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1750(t2274, t50, t7245, t9239);
    (t23278, t23281, t23290, t23295, t23383, t23508, t23598, t23788, t24498, t24514)
}
