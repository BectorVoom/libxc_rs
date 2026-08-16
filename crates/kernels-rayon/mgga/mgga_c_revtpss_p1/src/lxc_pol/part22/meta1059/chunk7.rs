//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3770/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3770(t12916: f64, t21041: f64, t3718: f64, t1214: f64, t20957: f64, t13396: f64, t17391: f64, t17396: f64, t17429: f64, t17534: f64, t17729: f64, t17736: f64, t17737: f64, t20858: f64, t20956: f64, t21035: f64, t21173: f64, t3367: f64, t3626: f64, t3720: f64, t4181: f64, t44500: f64, t44624: f64, t5245: f64, t5348: f64, t56953: f64, t59404: f64, t59406: f64, t59408: f64, t59415: f64) -> (f64, f64) {
    let t72017 = t3718 * t12916 * t21041;
    let t72044 = t20957 * t1214;
    let t72049 = -0.28582678745379824648e-3_f64 * t72017 + 0.20325460441158986416e-2_f64 * t59404 - 0.19055119163586549765e-3_f64 * t59406 - 0.3811023832717309953e-3_f64 * t59408 + 0.57165357490759649296e-3_f64 * t59415 + 0.11433071498151929859e-2_f64 * t17729 * t3626 * t5245 * t3367 * t4181 - 0.11433071498151929859e-2_f64 * t17736 * t3626 * t17737 * t17534 + 0.11433071498151929859e-2_f64 * t17729 * t3626 * t21035 * t13396 + 0.85748036236139473944e-3_f64 * t44624 * t20858 + 0.45732285992607719436e-2_f64 * t56953 * t5348 + 0.45732285992607719436e-2_f64 * t17396 * t17391 + 0.28582678745379824648e-3_f64 * t17429 * t21173 - 0.25724410870841842184e-2_f64 * t44500 * t3720 * t20956 * t72044;
    (t72044, t72049)
}
