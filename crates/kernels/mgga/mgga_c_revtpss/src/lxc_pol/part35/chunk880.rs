//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 880/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk880<F: Float>(t30: F, t33: F, t162: F, t22777: F, t22787: F, t189: F, t512: F, t1344: F, t22670: F, t22769: F, t5574: F, t5824: F, t9605: F, t1348: F, t22778: F, t22783: F, t5582: F, t6416: F, t9617: F, zeta_threshold: F) -> (F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t22789 = (t22777 + t22787) * t162;
    let t22790 = t22789 * t189;
    let t22791 = t512 * t22790;
    let t22799 = piecewise3::<f64>(t31, F::new(0.0), F::new(8.0) / F::new(27.0) * t9605 * t22769 - F::new(2.0) / F::new(3.0) * t5574 * t5824 + F::new(2.0) / F::new(3.0) * t1344 * t22670);
    let t22807 = piecewise3::<f64>(t34, F::new(0.0), F::new(8.0) / F::new(27.0) * t9617 * t22778 - F::new(2.0) / F::new(3.0) * t5582 * t6416 + F::new(2.0) / F::new(3.0) * t1348 * t22783);
    (t22789, t22791, t22799, t22807)
}
