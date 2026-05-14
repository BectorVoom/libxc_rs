//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1076/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1076<F: Float>(t126397: F, t121870: F, t122035: F, t122038: F, t126390: F, t126394: F, t126399: F, t126403: F, t27287: F, t27292: F, t31812: F, t32434: F, t34053: F, t7770: F, t7775: F, t8649: F, t886: F) -> (F,) {
    let t127871 = 0.14874931683620404328e-3 * t126397;
    let t127888 = 0.14874931683620404328e-2 * t126390 + 0.14874931683620404328e-2 * t126394 - t127871 + 0.26447628533477078895e-3 * t126399 - 0.56468933516960933999e-3 * t126403 + 0.8673628188205199462e0 * t121870 * t7775 + 0.8673628188205199462e0 * t32434 * t27287 + 0.8673628188205199462e0 * t32434 * t27292 - 0.17135921299530705785e1 * t8649 * t31812 * t34053 * t886 - 0.28912093960683998208e-1 * t122035 + 0.51405703062096148812e-1 * t122038 + 0.17347256376410398924e1 * t121870 * t7770;
    (t127888,)
}
