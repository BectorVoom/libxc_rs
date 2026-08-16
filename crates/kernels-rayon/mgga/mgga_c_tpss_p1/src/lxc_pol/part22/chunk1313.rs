//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1313/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1313(t17974: f64, t3689: f64, t10623: f64, t5559: f64, t1385: f64, t61086: f64, t17946: f64, t3622: f64, t10632: f64, t5547: f64, t10674: f64, t17960: f64, t3667: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t63960 = t17974 * t3689;
    let t63962 = t5559 * t10623;
    let t63964 = t61086 * t1385;
    let t63966 = t17946 * t3622;
    let t63968 = t5547 * t10632;
    let t63971 = t5559 * t10674;
    let t63973 = t17960 * t3667;
    (t63960, t63962, t63964, t63966, t63968, t63971, t63973)
}
