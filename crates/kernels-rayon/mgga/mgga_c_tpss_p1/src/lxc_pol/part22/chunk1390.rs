//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1390/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1390(t67223: f64, t67274: f64, t67589: f64, t67633: f64, t67674: f64, t67715: f64, t67751: f64, t67792: f64, t1279: f64, t13279: f64, t13283: f64, t13286: f64, t13289: f64, t1338: f64, t1668: f64, t1670: f64, t1851: f64, t19023: f64, t19037: f64, t19040: f64, t20678: f64, t20679: f64, t20694: f64, t3403: f64, t3407: f64, t3537: f64, t4559: f64, t547: f64, t548: f64, t5947: f64, t63152: f64, t6446: f64, t6455: f64, t66195: f64, t66199: f64, param_d: f64) -> (f64, f64) {
    let t67795 = t67223 + t67274 + t67589 + t67633 + t67674 + t67715 + t67751 + t67792;
    let t67800 = 6.0_f64 * t1338 * t547 * t63152 + 6.0_f64 * t1338 * t547 * t66195 + 12.0_f64 * t1338 * t547 * t66199 + 12.0_f64 * t19040 * t3537 * t547 + 12.0_f64 * t20678 * t3537 * t547 + t548 * t67795 * param_d + 12.0_f64 * t1279 * t20679 + 6.0_f64 * t1279 * t20694 + 6.0_f64 * t13279 * t1851 + 12.0_f64 * t13283 * t1851 + 6.0_f64 * t13286 * t1851 + 3.0_f64 * t13289 * t1851 + 6.0_f64 * t1668 * t19037 + 3.0_f64 * t1670 * t19023 + 3.0_f64 * t3403 * t6455 + 6.0_f64 * t3407 * t6446 + 6.0_f64 * t4559 * t5947;
    (t67795, t67800)
}
