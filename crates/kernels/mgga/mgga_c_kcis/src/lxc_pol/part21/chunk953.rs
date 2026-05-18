//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 953/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk953<F: Float>(t14477: F, t2630: F, t9933: F, t14439: F, t14442: F, t14446: F, t14450: F, t14455: F, t14460: F, t14463: F, t14467: F, t14470: F, t14474: F, t1706: F, t2867: F, t2872: F, t4953: F, t4968: F, t991: F) -> F {
    let t14478 = t14477 * t2630;
    let t14479 = t9933 * t14478;
    let t14482 = -F::new(11.0) / F::new(108.0) * t2867 * t1706 + t14439 - t14442 - t14446 + t14450 - t2872 * t4953 / F::new(27.0) - F::new(7.0) / F::new(432.0) * t14455 - t2872 * t4968 / F::new(9.0) + t991 * t14460 / F::new(48.0) + t991 * t14463 / F::new(48.0) + t991 * t14467 / F::new(144.0) - t991 * t14470 / F::new(36.0) - t991 * t14474 / F::new(288.0) - t991 * t14479 / F::new(216.0);
    t14482
}
