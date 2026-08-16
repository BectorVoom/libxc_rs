//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta575 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1856;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1857;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta575(t13231: f64, t25084: f64, t13353: f64, t23146: f64, t13225: f64, t23069: f64, t4159: f64, t23062: f64, t25106: f64, t13176: f64, t6613: f64, t831: f64, t25146: f64, t2681: f64, t23133: f64, t4257: f64, t1496: f64, t81942: f64, t7497: f64, t81933: f64, t25098: f64, t81835: f64, t13228: f64, t2628: f64, t2678: f64, t6605: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t87284, t87287, t87289, t87291, t87293, t87296) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1856(t13231, t25084, t13353, t23146, t13225, t23069, t4159, t23062, t25106, t13176, t6613, t831);
        let (t87298, t87300, t87304, t87306, t87308, t87312) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1857(t25146, t2681, t23133, t4257, t1496, t81942, t7497, t81933, t25098, t81835, t13228, t2628, t2678, t6605);
    (t87284, t87287, t87289, t87291, t87293, t87296, t87298, t87300, t87304, t87306, t87308, t87312)
}
