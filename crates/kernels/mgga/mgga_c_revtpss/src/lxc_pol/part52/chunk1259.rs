//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1259/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1259<F: Float>(t102622: F, t121059: F, t121102: F, t121109: F, t121112: F, t121118: F, t122358: F, t122455: F, t125671: F, t125677: F, t125706: F, t32719: F, t34212: F, t7925: F) -> F {
    let t128688 = F::new(0.25702851531048074406e-1) * t122358 - F::new(0.29749863367240808656e-2) * t125671 + F::new(0.225875734067843736e-2) * t125677 + t121059 - t121102 - F::new(0.34708173928447610099e-2) * t125706 - t121109 + t121112 + t121118 - F::new(0.11423947533020470523e1) * t122455 * t34212 - F::new(0.11423947533020470523e1) * t32719 * t102622 * t7925;
    t128688
}
