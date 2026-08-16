//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2457/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2457<F: Float>(t18322: F, t18791: F, t18810: F, t18836: F, t10563: F, t10566: F, t14324: F, t14343: F, t14345: F, t14372: F, t18392: F, t18535: F, t18536: F, t18537: F, t18538: F, t18541: F, t18543: F, t18546: F, t18548: F, t18549: F, t18552: F, t198: F, t207: F, t2403: F, t4343: F, t4546: F, t765: F, t892: F, t9394: F) -> (F, F) {
    let t18838 = t18322 + t18791 + t18810 + t18836;
    let t18848 = t18838 * t198 * t207 * t892 + F::cast_from(3.0_f64) * t18392 * t198 * t765 + F::cast_from(6.0_f64) * t2403 * t4343 * t4546 + t10563 + t10566 - t14324 + t14343 + t14345 + t14372 + t18535 - t18536 - t18537 + t18538 + t18541 + t18543 + t18546 + t18548 + t18549 + t18552 + t9394;
    (t18838, t18848)
}
