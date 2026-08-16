//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1942/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1942(t1353: f64, t22496: f64, t13514: f64, t93: f64, t116: f64, t28683: f64, t2055: f64, t2371: f64, t1459: f64, t1461: f64, t1518: f64, t18214: f64, t1916: f64, t1918: f64, t2113: f64, t2327: f64, t26716: f64, t26730: f64, t26734: f64, t26737: f64, t28956: f64, t28975: f64, t28978: f64, t28981: f64, t28986: f64, t4158: f64, t4165: f64, t572: f64, t5795: f64, t670: f64, t7554: f64, t7983: f64, t8118: f64, t8124: f64, t8127: f64) -> (f64, f64, f64) {
    let t101479 = t22496 * t1353;
    let t101522 = t93 * t13514;
    let t101705 = t116 * t28683;
    let t101720 = t2371 * t2055;
    let t101724 = 12.0_f64 * t101705 * t572 * t670 + 6.0_f64 * t101720 * t1518 * t572 + 6.0_f64 * t2327 * t572 * t7983 + 6.0_f64 * t2371 * t28986 * t572 + 12.0_f64 * t1459 * t28975 + 12.0_f64 * t1459 * t28978 + 12.0_f64 * t1459 * t28981 + 6.0_f64 * t1461 * t28956 + 3.0_f64 * t18214 * t2113 + 6.0_f64 * t1916 * t26730 + 12.0_f64 * t1916 * t26734 + 6.0_f64 * t1916 * t26737 + 3.0_f64 * t1918 * t26716 + 6.0_f64 * t4158 * t8124 + 3.0_f64 * t4158 * t8127 + 3.0_f64 * t4165 * t8118 + 12.0_f64 * t5795 * t7554;
    (t101479, t101522, t101724)
}
