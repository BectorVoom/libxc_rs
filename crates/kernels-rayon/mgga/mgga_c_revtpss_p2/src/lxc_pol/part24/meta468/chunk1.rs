//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1445/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1445(t40834: f64, t61837: f64, t854: f64, t10886: f64, t18608: f64, t808: f64, t18352: f64, t2710: f64, t2713: f64, t10722: f64, t6030: f64, t18419: f64, t9775: f64) -> (f64, f64, f64, f64, f64) {
    let t61839 = t40834 * t854 * t61837;
    let t61877 = t10886 * t808 * t18608;
    let t61888 = t2710 * t2713 * t18352;
    let t61890 = t10722 * t6030;
    let t61892 = t9775 * t18419;
    (t61839, t61877, t61888, t61890, t61892)
}
