//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1818/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1818<F: Float>(t30: F, t48292: F, t48294: F, t85929: F, t85931: F, t21906: F, t22670: F, t3833: F, t47025: F, t513: F, t5549: F, t5824: F, t87125: F, t91797: F, t91802: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t91982 = F::cast_from(960.0_f64) * t48292;
    let t91983 = F::cast_from(480.0_f64) * t48294;
    let t91984 = F::cast_from(16.0_f64) * t85929;
    let t91985 = F::cast_from(16.0_f64) * t85931;
    let t91997 = piecewise3::<F>(t31, F::cast_from(0.0_f64), F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t47025 * t91797 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t21906 * t5824 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t3833 * t91802 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t5549 * t22670 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t513 * t87125);
    (t91982, t91983, t91984, t91985, t91997)
}
