//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1057/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1057(t1526: f64, t6895: f64, t9483: f64, t33543: f64, t35757: f64, t1774: f64, t6903: f64, t7426: f64, t342: f64, t35772: f64, t630: f64, t10915: f64, t13616: f64, t1403: f64, t141478: f64, t141489: f64, t141491: f64, t141509: f64, t1424: f64, t15567: f64, t231: f64, t2320: f64, t27475: f64, t27483: f64, t27742: f64, t27781: f64, t27829: f64, t27833: f64, t27884: f64, t27892: f64, t2917: f64, t343: f64, t3691: f64, t3700: f64, t461: f64, t5996: f64, t6141: f64, t6745: f64, t6900: f64, t7150: f64, t7427: f64) -> f64 {
    let t151144 = t1526 * t9483 * t6895;
    let t151158 = t35757 * t33543;
    let t151167 = t7426 * t1774 * t6903;
    let t151183 = t342 * t630 * t35772;
    let t151188 = t1526 * t13616 * t27475 / 6.0_f64 - t151144 / 36.0_f64 + t5996 * t6900 / 3.0_f64 + t1403 * t27833 / 3.0_f64 + t1403 * t27884 / 3.0_f64 + t1403 * t27829 / 3.0_f64 - t141478 / 54.0_f64 - t7426 * t461 * t27781 / 6.0_f64 + t151158 / 18.0_f64 - t27892 * t7150 * t7427 / 6.0_f64 - t1526 * t2320 * t27483 / 12.0_f64 + t151167 / 18.0_f64 - t342 * t343 * t231 * t27742 / 4.0_f64 + t15567 * t2917 * t1424 * t3700 / 6.0_f64 - t15567 * t10915 * t1424 * t3691 / 9.0_f64 + t141489 - t141491 / 12.0_f64 - t151183 / 12.0_f64 - t141509 / 9.0_f64 + t6745 * t6141 / 3.0_f64;
    t151188
}
