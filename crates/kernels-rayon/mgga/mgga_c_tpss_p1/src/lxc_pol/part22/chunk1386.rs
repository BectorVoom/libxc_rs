//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1386/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1386(t10456: f64, t10464: f64, t1273: f64, t13133: f64, t13235: f64, t1760: f64, t1800: f64, t18544: f64, t19005: f64, t19577: f64, t19579: f64, t20218: f64, t20224: f64, t20289: f64, t20294: f64, t20322: f64, t20343: f64, t20357: f64, t20396: f64, t2056: f64, t2106: f64, t3499: f64, t3502: f64, t3538: f64, t41905: f64, t42719: f64, t5706: f64, t5757: f64, t5801: f64, t5809: f64, t5939: f64, t6243: f64, t6318: f64, t6328: f64, t6439: f64, t65052: f64, t7798: f64) -> f64 {
    let t67674 = -2.0_f64 * t7798 * t6318 - 4.0_f64 * t10456 * t6318 - 4.0_f64 * t2056 * t20396 - 4.0_f64 * t20294 * t3538 - 2.0_f64 * t5801 * t10464 - t18544 * t6439 - 2.0_f64 * t1760 * t20218 * t5757 + 2.0_f64 * t19579 * t20357 * t65052 - 2.0_f64 * t5706 * t20224 + 2.0_f64 * t20322 * t1273 - 4.0_f64 * t20294 * t3502 - 2.0_f64 * t20289 * t2106 - 2.0_f64 * t41905 * t1800 - 4.0_f64 * t42719 * t1800 - 4.0_f64 * t13133 * t5809 - 2.0_f64 * t13235 * t6328 - 4.0_f64 * t3499 * t20343 - t6243 * t19005 - 2.0_f64 * t19577 * t5939;
    t67674
}
