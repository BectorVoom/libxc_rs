//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1388/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1388(t10461: f64, t12664: f64, t13133: f64, t13235: f64, t1339: f64, t13554: f64, t1663: f64, t1796: f64, t1800: f64, t1846: f64, t18613: f64, t18919: f64, t19308: f64, t19577: f64, t20219: f64, t20386: f64, t20396: f64, t3396: f64, t3499: f64, t42336: f64, t485: f64, t5706: f64, t5801: f64, t5809: f64, t5816: f64, t5937: f64, t6103: f64, t62230: f64, t6318: f64, t6324: f64, t6409: f64, t65067: f64, t65941: f64, t67316: f64) -> f64 {
    let t67751 = -2.0_f64 * t67316 * t485 - 2.0_f64 * t13235 * t6324 - 4.0_f64 * t3499 * t20386 - 2.0_f64 * t62230 * t1339 + t6409 * t3396 + t18919 * t1663 - 2.0_f64 * t6103 * t18613 - 4.0_f64 * t13133 * t5816 - 2.0_f64 * t42336 * t1800 - 4.0_f64 * t13554 * t5809 - 2.0_f64 * t13235 * t6318 - 4.0_f64 * t3499 * t20396 - 2.0_f64 * t65067 * t1800 - 4.0_f64 * t19308 * t5809 + 2.0_f64 * t5706 * t20219 - t1796 * t12664 + 2.0_f64 * t19577 * t5937 + t65941 * t1846 - 4.0_f64 * t5801 * t10461;
    t67751
}
