//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1444/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1444(t10722: f64, t5993: f64, t40593: f64, t6037: f64, t124: f64, t6016: f64, t10744: f64, t18418: f64, t808: f64, t10886: f64, t18599: f64, t1544: f64, t1559: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t61677 = t10722 * t5993;
    let t61699 = t40593 * t6037;
    let t61715 = t124 * t6016;
    let t61797 = t10744 * t808 * t18418;
    let t61833 = t10886 * t808 * t18599;
    let t61837 = t1559 * t1544;
    (t61677, t61699, t61715, t61797, t61833, t61837)
}
