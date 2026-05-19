//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 407/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk407<F: Float>(t33: F, t1113: F, t1348: F, t1347: F, zeta_threshold: F) -> F {
    let t34 = t33 <= zeta_threshold;
    let t1351 = piecewise3::<F>(t34, F::new(0.0), F::new(2.0) / F::new(3.0) * t1348 * t1113);
    let t1353 = t1347 / F::new(2.0) + t1351 / F::new(2.0);
    t1353
}
