//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1308/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1308(t13003: f64, t13028: f64, t252: f64, t1492: f64, t2710: f64, t1519: f64, t2591: f64, t225: f64, t4266: f64, t10049: f64, t1528: f64, t259: f64, t2597: f64, t2713: f64, t2720: f64, t2743: f64, t4147: f64, t4268: f64, t4273: f64, t4301: f64, t866: f64, t9590: f64, t9593: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13029 = t13003 + t13028;
    let t13030 = t13029 * t252;
    let t13034 = t1492 * t2710;
    let t13036 = t2591 * t1519;
    let t13042 = t4266 * t225;
    let t13048 = -t10049 * t1528 + t13030 * t259 + t13034 * t259 + t13036 * t259 - 2.0_f64 * t13042 * t866 - t1528 * t9590 - 2.0_f64 * t1528 * t9593 - 2.0_f64 * t2597 * t4301 + 4.0_f64 * t2713 * t4273 + 2.0_f64 * t2720 * t4147 - t2743 * t4147 - t2743 * t4268;
    (t13029, t13030, t13034, t13036, t13042, t13048)
}
