//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta442 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1691;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1692;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta442(t3792: f64, t6414: f64, t2632: f64, t5611: f64, t107: f64, t240: f64, t625: f64, t656: f64, t666: f64, t2331: f64, t63: f64, t2240: f64, t608: f64, t1864: f64, t645: f64, t192: f64, t532: f64, t1982: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20473, t20986, t22469, t22470, t22471, t22473, t22510, t22549) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1691(t3792, t6414, t2632, t5611, t107, t240, t625, t656, t666, t2331, t63, t2240, t608);
        let (t22550, t22573, t22574) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1692(t1864, t645, t192, t532, t1982);
    (t20473, t20986, t22469, t22470, t22471, t22473, t22510, t22549, t22550, t22573, t22574)
}
