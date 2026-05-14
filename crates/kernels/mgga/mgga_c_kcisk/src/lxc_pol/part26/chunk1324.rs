//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1324/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1324<F: Float>(t1333: F, t34736: F, t1339: F, t33604: F, t5621: F, t1411: F, t26774: F, t32045: F, t1308: F, t27016: F, t388: F, t110505: F, t110509: F, t113947: F, t114075: F, t114157: F, t118853: F, t118933: F, t33346: F, t33377: F, t33384: F, t33428: F, t9429: F, t9446: F, t9796: F) -> (F, F, F, F, F) {
    let t119154 = t1333 * t34736;
    let t119162 = t1339 * t33604 * t5621;
    let t119169 = t1411 * t32045 * t26774;
    let t119174 = t27016 * t388 * t1308;
    let t119179 = 0.89351851851851851853e-3 * t114157 + 0.16581944444444444444e-2 * t119154 - 0.61728395061728395064e-2 * t110505 - t110509 + 0.20833333333333333334e-1 * t114075 * t9796 + 0.8041666666666666667e-2 * t33377 * t33346 + 0.22109259259259259259e-2 * t119162 + 0.26805555555555555557e-2 * t113947 * t33428 - 0.20833333333333333334e-1 * t9446 * t118853 + 0.11054629629629629629e-2 * t119169 + 0.20833333333333333334e-1 * t33384 * t33346 + 0.10416666666666666667e-1 * t119174 * t9429 - 0.20833333333333333334e-1 * t9446 * t118933;
    (t119154, t119162, t119169, t119174, t119179)
}
