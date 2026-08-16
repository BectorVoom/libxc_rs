//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1855/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1855(t1527: f64, t857: f64, t776: f64, t23270: f64, t22986: f64, t225: f64, t258: f64, t4265: f64, t214: f64, t1880: f64, t1484: f64, t22690: f64, t841: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t25053 = t857 * t1527;
    let t25054 = t25053 * t776;
    let t25055 = t23270 * t25054;
    let t25056 = t22986 * t25055;
    let t25059 = t4265 * t225 * t258;
    let t25060 = t214 * t25059;
    let t25061 = t1880 * t25060;
    let t25064 = t22690 * t841 * t1484;
    (t25053, t25054, t25055, t25056, t25059, t25060, t25061, t25064)
}
