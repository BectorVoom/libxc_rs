//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3770/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3770<F: Float>(t12916: F, t21041: F, t3718: F, t1214: F, t20957: F, t13396: F, t17391: F, t17396: F, t17429: F, t17534: F, t17729: F, t17736: F, t17737: F, t20858: F, t20956: F, t21035: F, t21173: F, t3367: F, t3626: F, t3720: F, t4181: F, t44500: F, t44624: F, t5245: F, t5348: F, t56953: F, t59404: F, t59406: F, t59408: F, t59415: F) -> (F, F) {
    let t72017 = t3718 * t12916 * t21041;
    let t72044 = t20957 * t1214;
    let t72049 = -F::cast_from(0.28582678745379824648e-3_f64) * t72017 + F::cast_from(0.20325460441158986416e-2_f64) * t59404 - F::cast_from(0.19055119163586549765e-3_f64) * t59406 - F::cast_from(0.3811023832717309953e-3_f64) * t59408 + F::cast_from(0.57165357490759649296e-3_f64) * t59415 + F::cast_from(0.11433071498151929859e-2_f64) * t17729 * t3626 * t5245 * t3367 * t4181 - F::cast_from(0.11433071498151929859e-2_f64) * t17736 * t3626 * t17737 * t17534 + F::cast_from(0.11433071498151929859e-2_f64) * t17729 * t3626 * t21035 * t13396 + F::cast_from(0.85748036236139473944e-3_f64) * t44624 * t20858 + F::cast_from(0.45732285992607719436e-2_f64) * t56953 * t5348 + F::cast_from(0.45732285992607719436e-2_f64) * t17396 * t17391 + F::cast_from(0.28582678745379824648e-3_f64) * t17429 * t21173 - F::cast_from(0.25724410870841842184e-2_f64) * t44500 * t3720 * t20956 * t72044;
    (t72044, t72049)
}
