//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1186/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1186<F: Float>(t120199: F, t33425: F, t34904: F, t33397: F, t34990: F, t33404: F, t1263: F, t1828: F, t1042: F, t1122: F, t1203: F, t1214: F, t124594: F, t124646: F, t124780: F, t1248: F, t124945: F, t124984: F, t1287: F, t13141: F, t131416: F, t2148: F, t247: F, t29124: F, t33401: F, t33405: F, t33408: F, t33461: F, t33462: F, t33469: F, t33478: F, t34908: F, t34914: F, t34939: F, t34960: F, t3719: F, t494: F, t5245: F, t5342: F) -> F {
    let t131734 = t33425 * t120199 * t34904;
    let t131745 = t33397 * t34990;
    let t131748 = t33404 * t34990;
    let t131766 = t1263 * t1828;
    let t131771 = F::cast_from(0.3427184259906141157e1_f64) * t124945 * t34914 * t1248 * t1287 - F::cast_from(0.12548651892657985333e-3_f64) * t131734 - F::cast_from(0.3427184259906141157e1_f64) * t124984 * t34939 * t1248 * t1287 - F::cast_from(0.56468933516960933998e-3_f64) * t33405 * t247 * t3719 * t494 * t5245 - F::cast_from(0.30116764542379164798e-2_f64) * t131745 * t33401 + F::cast_from(0.30116764542379164798e-2_f64) * t131748 * t33408 - F::cast_from(0.34271842599061411569e1_f64) * t33469 * t33462 * t34908 * t1214 - F::cast_from(0.51407763898592117355e1_f64) * t33461 * t33478 * t34960 * t1203 - F::cast_from(0.17347256376410398924e1_f64) * t124780 * t29124 + F::cast_from(0.56468933516960933998e-3_f64) * t2148 * t13141 * t131416 * t124646 * t5342 - F::cast_from(0.24791552806034007213e-3_f64) * t124594 * t1042 * t131766 * t1122;
    t131771
}
