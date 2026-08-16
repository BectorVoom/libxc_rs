//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 712/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk712(t1691: f64, t713: f64, t5270: f64, t721: f64, t1923: f64, t1945: f64, t1957: f64, t1981: f64, t2017: f64, t202: f64, t208: f64, t390: f64, t5444: f64, t5451: f64, t5513: f64, t5524: f64, t5527: f64, t5530: f64, t5531: f64, t5534: f64, t5537: f64, t5539: f64, t5543: f64, t5549: f64, t674: f64, t705: f64, t718: f64) -> f64 {
    let t5556 = t713 * t1691;
    let t5559 = t721 * t5270;
    let t5562 = -0.30822e0_f64 * t390 * t5513 + 1.0_f64 * t202 * t5524 - 0.31168546390226634765e3_f64 * t1945 * t5527 - 0.12304822629859687989e5_f64 * t5530 * t5531 - 0.35089341735807877242e1_f64 * t705 * t5534 + t5444 + 0.91082604192152556044e5_f64 * t5537 * t5539 + 0.51947577317044391277e2_f64 * t718 * t5543 - 2.0_f64 * t674 * t208 * t5549 - 0.57895126195293126242e3_f64 * t1957 * t2017 * t1923 + 0.10526802520742363173e2_f64 * t718 * t5556 + 0.6233709278045326953e3_f64 * t1981 * t5559 + t5451;
    t5562
}
