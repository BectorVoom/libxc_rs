//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1376/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1376<F: Float>(t33: F, t6792: F, t9350: F, t3841: F, t6416: F, t1113: F, t20256: F, t2255: F, t516: F, t5557: F, t162: F, t21917: F, t187: F, zeta_threshold: F) -> (F, F) {
    let t34 = t33 <= zeta_threshold;
    let t21918 = t9350 * t6792;
    let t21923 = t3841 * t6416;
    let t21929 = piecewise3::<f64>(t34, F::new(0.0), -F::new(8.0) / F::new(27.0) * t21918 * t1113 - F::new(16.0) / F::new(9.0) * t5557 * t2255 + F::new(4.0) / F::new(9.0) * t21923 * t1113 + F::new(4.0) / F::new(3.0) * t516 * t20256);
    let t21931 = (t21917 + t21929) * t162;
    let t21933 = F::new(0.19751673498613801407e-1) * t21931 * t187;
    (t21931, t21933)
}
