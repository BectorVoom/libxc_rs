//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 499/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk499<F: Float>(t265: F, t502: F, t1210: F, t1274: F, t1770: F, t1775: F, t1813: F, t1829: F, t460: F, t495: F, t1300: F, t1587: F, t1721: F, t1735: F, t1761: F, t1763: F, t1767: F, t198: F, t336: F) -> (F, F) {
    let t503 = t265 < t502;
    let t1832 = F::cast_from(0.65854491829355115987e0_f64) * t1770 * t495 - F::cast_from(0.65854491829355115987e0_f64) * t1210 * t1775 + F::cast_from(0.65854491829355115987e0_f64) * t460 * t1813 - F::cast_from(0.65854491829355115987e0_f64) * t1274 * t1829;
    let t1837 = piecewise3::<F>(t503, t1300 * t1832 * t198 * t336 - t1721 + t1735 + t1761 + t1763 - t1767, t1587);
    (t1832, t1837)
}
