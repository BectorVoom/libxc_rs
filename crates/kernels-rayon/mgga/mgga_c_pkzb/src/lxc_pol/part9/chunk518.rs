//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 518/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk518(t133: f64, t2009: f64, t793: f64, t2036: f64, t306: f64, t2126: f64, t287: f64, t2124: f64, t2111: f64, t2123: f64, t2128: f64, t2131: f64, t290: f64, t791: f64, t794: f64) -> (f64, f64, f64, f64) {
    let t2134 = t2009 * t133;
    let t2135 = t2134 * t793;
    let t2138 = t2036 * t306;
    let t2139 = t2126 * t287;
    let t2140 = t2124 * t2139;
    let t2145 = 0.13170898365871023197e1_f64 * t2123 * t2128 + 0.13170898365871023197e1_f64 * t2131 * t794 + 0.65854491829355115987e0_f64 * t791 * t2135 - 0.65854491829355115987e0_f64 * t2138 * t2140 + 0.65854491829355115987e0_f64 * t290 * t2111;
    (t2134, t2135, t2140, t2145)
}
