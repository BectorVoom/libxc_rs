//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 318/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk318<F: Float>(t169: F, t1503: F, t1556: F, t1625: F, t1629: F, t1636: F, t187: F, t633: F, t828: F, t89: F, t171: F, zeta_threshold: F) -> (F, F, F, F) {
    let t170 = t169 <= zeta_threshold;
    let t1640 = t1503 - t1556 + t187 * (t1625 * t633 - t1629 * t1636 - t1503 + t1556);
    let t1646 = -t89 - t828;
    let t1649 = piecewise3::<F>(t170, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t171 * t1646);
    let t1650 = -t1646;
    (t1640, t1646, t1649, t1650)
}
