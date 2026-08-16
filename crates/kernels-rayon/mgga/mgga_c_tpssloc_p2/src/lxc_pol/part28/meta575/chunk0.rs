//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1856/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1856(t13231: f64, t25084: f64, t13353: f64, t23146: f64, t13225: f64, t23069: f64, t4159: f64, t23062: f64, t25106: f64, t13176: f64, t6613: f64, t831: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t87284 = t25084 * t13231;
    let t87287 = t23146 * t13353;
    let t87289 = t23146 * t13225;
    let t87291 = t23069 * t4159;
    let t87293 = t23062 * t25106;
    let t87295 = t13176 * t6613;
    let t87296 = t87295 * t831;
    (t87284, t87287, t87289, t87291, t87293, t87296)
}
