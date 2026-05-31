//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 449/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk449<F: Float>(t30: F, t33: F, t1857: F, t512: F, t1856: F, t187: F, t1344: F, t1468: F, t1348: F, t1711: F, zeta_threshold: F) -> (F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t1858 = t512 * t1857;
    let t1860 = F::cast_from(0.19751673498613801407e-1_f64) * t1856 * t187;
    let t1863 = piecewise3::<F>(t31, F::cast_from(0.0_f64), F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1344 * t1468);
    let t1866 = piecewise3::<F>(t34, F::cast_from(0.0_f64), F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1348 * t1711);
    let t1868 = t1863 / F::cast_from(2.0_f64) + t1866 / F::cast_from(2.0_f64);
    (t1858, t1860, t1868)
}
