//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta253 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1229;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1230;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1231;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1232;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1233;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta253(t1307: f64, t1390: f64, t6878: f64, t1983: f64, t1984: f64, t6546: f64, t1988: f64, t131: f64, t209: f64, t547: f64, t1878: f64, t214: f64, t562: f64, t225: f64, t567: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6879, t6880, t6882, t6883) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1229(t1307, t1390, t6878, t1983, t1984, t6546);
        let (t6885, t6887, t6888) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1230(t1988, t6883, t131, t209, t547, t1878);
        let t6889 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1231(t214, t562);
        let t6890 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1232(t225, t567);
        let t6891 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1233(t1307, t6890);
    (t6879, t6880, t6882, t6883, t6885, t6887, t6888, t6889, t6890, t6891)
}
