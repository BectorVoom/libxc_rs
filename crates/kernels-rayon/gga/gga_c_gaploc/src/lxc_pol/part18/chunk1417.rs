//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1417/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1417(t10364: f64, t1562: f64, t4614: f64, t1445: f64, t31711: f64, t597: f64, t31866: f64, t10123: f64, t10151: f64, t10480: f64, t1305: f64, t1429: f64, t1450: f64, t1456: f64, t1457: f64, t188: f64, t189: f64, t193: f64, t31291: f64, t31502: f64, t31509: f64, t31623: f64, t31655: f64, t34983: f64, t34986: f64, t34991: f64, t34994: f64, t34996: f64, t4679: f64, t549: f64, t567: f64) -> f64 {
    let t34999 = 0.18404604457881959845e2_f64 * t1562 * t4614 * t10364;
    let t35021 = 0.23005755572352449806e2_f64 * t597 * t1445 * t31711;
    let t35024 = 0.11502877786176224903e2_f64 * t597 * t1445 * t31866;
    let t35025 = t34983 - t34986 + 0.79445533226334281486e-1_f64 * t1429 * t549 * t31655 + t34991 + t34994 - t34996 - t34999 + 0.61348681526273199482e1_f64 * t567 * t4614 * t10123 - t31291 + 0.35750489951850426669e0_f64 * t188 * t189 * t31623 * t193 + 0.71500979903700853338e0_f64 * t1456 * t1457 * t31502 + 0.71500979903700853338e0_f64 * t4679 * t10480 - 0.23005755572352449806e1_f64 * t1450 * t1445 * t10151 * t1305 + 0.23005755572352449806e1_f64 * t567 * t1445 * t31509 + t35021 + t35024;
    t35025
}
