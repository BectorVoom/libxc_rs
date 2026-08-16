//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta654 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1934;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1935;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta654(t23097: f64, t232: f64, t67783: f64, t815: f64, t16888: f64, t23146: f64, t16969: f64, t25146: f64, t4236: f64, t23053: f64, t5614: f64, t16859: f64, t6614: f64, t16673: f64, t6613: f64, t831: f64, t28359: f64, t838: f64, t23069: f64, t5572: f64, t23062: f64, t28383: f64, t20986: f64, t2628: f64, t6605: f64, t828: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98672, t98674, t98676, t98678, t98680, t98682) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1934(t23097, t232, t67783, t815, t16888, t23146, t16969, t25146, t4236, t23053, t5614, t16859, t6614);
        let (t98685, t98690, t98694, t98696, t98701) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1935(t16673, t6613, t831, t28359, t838, t23069, t5572, t23062, t28383, t20986, t2628, t6605, t828);
    (t98672, t98674, t98676, t98678, t98680, t98682, t98685, t98690, t98694, t98696, t98701)
}
