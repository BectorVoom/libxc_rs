//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 979/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk979<F: Float>(t1265: F, t31993: F, t1238: F, t1252: F, t33469: F, t33471: F, t33474: F, t33477: F, t33480: F, t33484: F, t33487: F, t33491: F, t33495: F, t33498: F, t33502: F, t33505: F, t33509: F, t33512: F, t33518: F, t33523: F, t33524: F, t8941: F, t8948: F) -> (F, F) {
    let t33525 = t31993 * t1265;
    let t33528 = -F::new(0.17135921299530705785e1) * t33469 * t33471 + F::new(0.57119737665102352616e0) * t33474 * t8941 - F::new(0.17135921299530705785e1) * t33477 * t33480 + F::new(0.11423947533020470523e1) * t33484 * t33487 + F::new(0.11423947533020470523e1) * t33477 * t33491 - F::new(0.5578099381357651623e-3) * t33495 * t33498 + F::new(0.5578099381357651623e-3) * t33502 * t1238 - F::new(0.1859366460452550541e-3) * t33505 * t8948 + F::new(0.3718732920905101082e-3) * t33509 * t33512 - F::new(0.3718732920905101082e-3) * t33518 * t1252 - t33523 + F::new(0.12395776403017003607e-3) * t33524 * t33525;
    (t33525, t33528)
}
