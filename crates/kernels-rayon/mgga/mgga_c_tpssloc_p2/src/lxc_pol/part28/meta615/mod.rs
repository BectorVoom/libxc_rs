//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta615 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1930;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1931;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta615(t26308: f64, t3777: f64, t5252: f64, t16257: f64, t26309: f64, t5293: f64, t80820: f64, t5259: f64, t80816: f64, t16244: f64, t22833: f64, t5303: f64, t16366: f64, t16370: f64, t26257: f64, t3872: f64, t1831: f64, t80869: f64, t22783: f64, t5314: f64, t26297: f64, t80853: f64, t80855: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t91116, t91118, t91120, t91122, t91124, t91126) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1930(t26308, t3777, t5252, t16257, t26309, t5293, t80820, t5259, t80816, t16244, t22833, t5303);
        let (t91128, t91130, t91133, t91135, t91137, t91140) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1931(t16366, t22833, t16370, t26257, t3872, t1831, t80869, t22783, t5314, t26297, t80853, t80855);
    (t91116, t91118, t91120, t91122, t91124, t91126, t91128, t91130, t91133, t91135, t91137, t91140)
}
