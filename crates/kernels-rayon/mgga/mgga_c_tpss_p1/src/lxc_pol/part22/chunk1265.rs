//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1265/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1265(t1338: f64, t5895: f64, t18690: f64, t19609: f64, t1844: f64, t9895: f64, t19581: f64, t5757: f64, t6436: f64, t13133: f64, t13554: f64, t1760: f64, t1800: f64, t18547: f64, t19305: f64, t19308: f64, t19579: f64, t20289: f64, t2056: f64, t3493: f64, t3499: f64, t5706: f64, t5809: f64, t5816: f64, t6103: f64, t626: f64, t6328: f64, t6439: f64, t646: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20343 = t5895 * t1338;
    let t20346 = t18690 * t19609;
    let t20357 = t1844 * t9895;
    let t20358 = t20357 * t19581;
    let t20361 = t6436 * t5757;
    let t20363 = -2.0_f64 * t13133 * t1800 - 2.0_f64 * t13554 * t1800 - t1760 * t20361 - 2.0_f64 * t1800 * t19305 - 2.0_f64 * t1800 * t19308 - 3.0_f64 * t18547 * t20346 + 2.0_f64 * t19579 * t20358 - 2.0_f64 * t20289 * t646 - 2.0_f64 * t20343 * t626 - 2.0_f64 * t2056 * t6328 - 2.0_f64 * t3493 * t5809 - 2.0_f64 * t3493 * t5816 - 2.0_f64 * t3499 * t6328 - t5706 * t6439 - 2.0_f64 * t5809 * t6103;
    (t20343, t20346, t20357, t20358, t20361, t20363)
}
