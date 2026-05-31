//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 958/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk958<F: Float>(t30: F, t33: F, t13611: F, t1468: F, t6785: F, t22670: F, t513: F, t5549: F, t5824: F, t9335: F, t1711: F, t6792: F, t516: F, t5557: F, t6416: F, t9350: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t22768 = F::cast_from(0.17544670867903938621e1_f64) * t13611;
    let t22769 = t6785 * t1468;
    let t22777 = piecewise3::<F>(t31, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t9335 * t22769 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t5549 * t5824 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t513 * t22670);
    let t22778 = t6792 * t1711;
    let t22783 = -t22670;
    let t22787 = piecewise3::<F>(t34, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t9350 * t22778 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t5557 * t6416 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t516 * t22783);
    (t22768, t22769, t22777, t22778, t22783, t22787)
}
