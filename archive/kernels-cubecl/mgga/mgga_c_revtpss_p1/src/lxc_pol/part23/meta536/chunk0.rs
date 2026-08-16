//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2071/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2071<F: Float>(t30: F, t21881: F, t508: F, t1518: F, t5517: F, t13584: F, t9375: F, t6785: F, t9335: F, t3833: F, t5824: F, t18280: F, t2255: F, t513: F, t5549: F, t605: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t21882 = t508 * t21881;
    let t21891 = t5517 * t1518;
    let t21901 = F::cast_from(40.0_f64) * t13584;
    let t21905 = F::cast_from(0.5848223622634646207e0_f64) * t9375;
    let t21906 = t9335 * t6785;
    let t21911 = t3833 * t5824;
    let t21917 = piecewise3::<F>(t31, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t21906 * t605 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t5549 * t2255 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t21911 * t605 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t513 * t18280);
    (t21882, t21891, t21901, t21905, t21906, t21917)
}
