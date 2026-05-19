//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 956/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk956<F: Float>(t154: F, t506: F, t7322: F, t7326: F, t7315: F, t8589: F, t30268: F, t8775: F, t30105: F, t8952: F, t7839: F, t8739: F) -> (F, F, F, F, F) {
    let t33960 = t7322 * t154 * t506 * t7326;
    let t33962 = t7315 * t8589;
    let t33963 = F::new(11.0) / F::new(192.0) * t33962;
    let t33982 = t30268 * t8775;
    let t33983 = F::cast_from(0.64311027177104605458e-2_f64) * t33982;
    let t33984 = t30105 * t8952;
    let t33986 = t7839 * t8739;
    (t33960, t33963, t33983, t33984, t33986)
}
