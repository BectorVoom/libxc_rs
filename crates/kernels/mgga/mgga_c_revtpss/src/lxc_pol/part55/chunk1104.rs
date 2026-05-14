//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1104/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1104<F: Float>(t122274: F, t27873: F, t125663: F, t121029: F, t121044: F, t122351: F, t122355: F, t122443: F, t125659: F, t128665: F, t32674: F, t32678: F, t34231: F, t7926: F, t102622: F, t121059: F, t121102: F, t121109: F, t121112: F, t121118: F, t122358: F, t122455: F, t125671: F, t125677: F, t125706: F, t32719: F, t34212: F, t7925: F) -> (F, F) {
    let t128673 = t122274 * t27873;
    let t128676 = 0.150583822711895824e-3 * t125663;
    let t128677 = -t122351 - 0.28912093960683998207e-1 * t128665 + 0.57119737665102352616e0 * t34231 * t32674 + 0.57119737665102352616e0 * t34231 * t32678 + t121029 + 0.8673628188205199462e0 * t122443 * t7926 + 0.51405703062096148813e-1 * t128673 - 0.225875734067843736e-2 * t125659 + t128676 - t121044 - t122355;
    let t128688 = 0.25702851531048074406e-1 * t122358 - 0.29749863367240808656e-2 * t125671 + 0.225875734067843736e-2 * t125677 + t121059 - t121102 - 0.34708173928447610099e-2 * t125706 - t121109 + t121112 + t121118 - 0.11423947533020470523e1 * t122455 * t34212 - 0.11423947533020470523e1 * t32719 * t102622 * t7925;
    (t128677, t128688)
}
