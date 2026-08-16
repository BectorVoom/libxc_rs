//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta641 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2356;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta641(t2495: f64, t9385: f64, t2491: f64, t744: f64, t760: f64, t2492: f64, t2514: f64, t9367: f64, t9371: f64, t200: f64, t631: f64, t202: f64, t635: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t39815, t39816, t39818, t39821, t39823, t39825, t39840) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2356(t2495, t9385, t2491, t744, t760, t2492, t2514, t9367, t9371, t200, t631, t202, t635);
    (t39815, t39816, t39818, t39821, t39823, t39825, t39840)
}
