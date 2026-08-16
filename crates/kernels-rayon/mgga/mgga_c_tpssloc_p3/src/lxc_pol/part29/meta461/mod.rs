//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta461 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1784;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1785;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1786;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta461(t23133: f64, t849: f64, t2707: f64, t6621: f64, t1891: f64, t9223: f64, t213: f64, t1895: f64, t1887: f64, t206: f64, t22715: f64, t242: f64, t6612: f64, t812: f64, t2649: f64, t23096: f64, t23100: f64, t23106: f64, t23108: f64, t23114: f64, t23117: f64, t23120: f64, t23125: f64, t23128: f64, t23130: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23134, t23135, t23136, t23138, t23141, t23143, t23144, t23145) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1784(t23133, t849, t2707, t6621, t1891, t9223, t213, t1895, t1887, t206, t22715, t242, t6612);
        let t23146 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1785(t23145, t812);
        let t23149 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1786(t23146, t2649, t23096, t23100, t23106, t23108, t23114, t23117, t23120, t23125, t23128, t23130, t23135, t23136, t23141, t23144);
    (t23134, t23138, t23141, t23143, t23144, t23145, t23146, t23149)
}
