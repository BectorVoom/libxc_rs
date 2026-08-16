//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1745/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1745<F: Float>(t1042: F, t17505: F, t1797: F, t21107: F, t24612: F, t3610: F, t3611: F, t5268: F, t5296: F, t5384: F, t5825: F, t6573: F, t6625: F, t6631: F, t6635: F, t71693: F, t71699: F, t82555: F, t82821: F, t82824: F, t82827: F, t90037: F, t90081: F) -> F {
    let t90245 = F::cast_from(0.86891343385954666928e-1_f64) * t71693 * t6631 - F::cast_from(0.34299214494455789578e-2_f64) * t5384 * t1042 * t5268 * t90037 - F::cast_from(0.17149607247227894789e-2_f64) * t5384 * t1042 * t5296 * t5825 * t6573 - F::cast_from(0.18292914397043087775e-1_f64) * t17505 * t24612 - F::cast_from(0.43445671692977333464e-1_f64) * t71699 * t6635 - F::cast_from(0.13719685797782315831e-1_f64) * t82555 * t1797 - F::cast_from(0.64311027177104605458e-3_f64) * t3610 * t1042 * t90081 * t3611 - F::cast_from(0.13719685797782315831e-1_f64) * t21107 * t6625 - F::cast_from(0.18292914397043087775e-1_f64) * t82821 + F::cast_from(0.11433071498151929859e-2_f64) * t82824 + F::cast_from(0.19055119163586549765e-2_f64) * t82827;
    t90245
}
