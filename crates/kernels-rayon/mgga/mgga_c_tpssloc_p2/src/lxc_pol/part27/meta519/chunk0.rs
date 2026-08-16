//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1924/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1924(t23384: f64, t7604: f64, t1615: f64, t6768: f64, t1060: f64, t2987: f64, t4343: f64, t4338: f64, t4509: f64, t4640: f64, t6754: f64, t1611: f64, t6764: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25563 = t23384 * t7604;
    let t25567 = t6768 * t1615;
    let t25568 = t25567 * t1060;
    let t25571 = t2987 * t4343;
    let t25574 = t4509 * t4338;
    let t25577 = t4640 * t6754;
    let t25580 = t1611 * t6764;
    (t25563, t25567, t25568, t25571, t25574, t25577, t25580)
}
