//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 599/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk599(t524: f64, t8396: f64, t1589: f64, t1586: f64, t1580: f64, t2318: f64, t2322: f64, t2328: f64, t4418: f64, t535: f64, t541: f64, t6456: f64, t6459: f64, t6474: f64, t6498: f64, t8308: f64, t8319: f64, t8324: f64, t8328: f64, t8332: f64, t8337: f64) -> (f64, f64, f64, f64) {
    let t536 = 0.0_f64 < t524;
    let t8398 = piecewise3(t536, t8396, -t8396);
    let t8399 = t1589 * t8398;
    let t8400 = t1586 * t8399;
    let t8403 = 0.2698618307426597582e-1_f64 * t8308 * t541 + 0.17990788716177317213e-1_f64 * t6456 + 0.17990788716177317213e-1_f64 * t6459 * t2322 - 0.5397236614853195164e-1_f64 * t2318 * t2328 - t4418 + 0.59969295720591057378e-2_f64 * t6474 - 0.17990788716177317213e-1_f64 * t6498 + 0.11993859144118211476e-1_f64 * t1580 * t8319 - 0.17990788716177317213e-1_f64 * t1580 * t8324 - 0.17990788716177317213e-1_f64 * t1580 * t8328 + 0.89953943580886586067e-2_f64 * t1580 * t8332 + 0.5397236614853195164e-1_f64 * t535 * t8337 - 0.2698618307426597582e-1_f64 * t535 * t8400;
    (t8398, t8399, t8400, t8403)
}
