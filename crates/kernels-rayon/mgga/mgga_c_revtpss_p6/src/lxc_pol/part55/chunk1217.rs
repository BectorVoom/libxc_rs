//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1217/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1217(t126397: f64, t121870: f64, t122035: f64, t122038: f64, t126390: f64, t126394: f64, t126399: f64, t126403: f64, t27287: f64, t27292: f64, t31812: f64, t32434: f64, t34053: f64, t7770: f64, t7775: f64, t8649: f64, t886: f64) -> f64 {
    let t127871 = 0.14874931683620404328e-3_f64 * t126397;
    let t127888 = 0.14874931683620404328e-2_f64 * t126390 + 0.14874931683620404328e-2_f64 * t126394 - t127871 + 0.26447628533477078895e-3_f64 * t126399 - 0.56468933516960933999e-3_f64 * t126403 + 0.8673628188205199462e0_f64 * t121870 * t7775 + 0.8673628188205199462e0_f64 * t32434 * t27287 + 0.8673628188205199462e0_f64 * t32434 * t27292 - 0.17135921299530705785e1_f64 * t8649 * t31812 * t34053 * t886 - 0.28912093960683998208e-1_f64 * t122035 + 0.51405703062096148812e-1_f64 * t122038 + 0.17347256376410398924e1_f64 * t121870 * t7770;
    t127888
}
