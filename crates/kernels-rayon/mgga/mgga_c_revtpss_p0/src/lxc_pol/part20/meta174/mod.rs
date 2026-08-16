//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta174 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk913;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta174(t1331: f64, t3857: f64, t189: f64, t9363: f64, t512: f64, t3850: f64, t72: f64, t757: f64, t2619: f64, t3825: f64, t1333: f64, t3863: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9560, t9561, t9562, t9563, t9565, t9567, t9569, t9570) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk913(t1331, t3857, t189, t9363, t512, t3850, t72, t757, t2619, t3825, t1333, t3863);
    (t9560, t9561, t9562, t9563, t9565, t9567, t9569, t9570)
}
