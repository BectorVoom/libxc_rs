//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1105/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1105(t28737: f64, t9797: f64, t2586: f64, t2679: f64, t9796: f64, t2013: f64, t9813: f64, t825: f64, t826: f64, t9829: f64, t15362: f64, t9810: f64) -> (f64, f64, f64, f64, f64) {
    let t28738 = t28737 * t9797;
    let t28739 = 0.1533717038156829987e1_f64 * t28738;
    let t28742 = t9796 * t2586 * t2679;
    let t28743 = 0.1533717038156829987e1_f64 * t28742;
    let t28792 = t2013 * t9813;
    let t28793 = 0.1022478025437886658e1_f64 * t28792;
    let t28795 = t825 * t826 * t9829;
    let t28796 = 0.1022478025437886658e1_f64 * t28795;
    let t28800 = 0.11916829983950142223e0_f64 * t15362 * t9810;
    (t28739, t28743, t28793, t28796, t28800)
}
