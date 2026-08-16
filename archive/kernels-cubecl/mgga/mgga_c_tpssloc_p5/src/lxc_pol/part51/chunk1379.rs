//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1379/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1379<F: Float>(t114866: F, t6552: F, t7479: F, t25341: F, t31366: F, t1880: F, t26679: F, t6553: F, t6571: F, t114592: F, t118476: F, t118479: F, t118481: F, t118484: F, t121296: F, t121299: F, t121302: F, t121305: F, t121308: F) -> F {
    let t121311 = t6552 * t114866 * t7479;
    let t121314 = t6552 * t31366 * t25341;
    let t121318 = t1880 * t6553 * t6571 * t26679;
    let t121320 = t118476 + t118479 - F::cast_from(0.82246703342411321824e-2_f64) * t114592 - t118481 + F::cast_from(0.19190897446562641759e-1_f64) * t121296 + F::cast_from(0.16449340668482264365e-1_f64) * t121299 + t118484 - F::cast_from(0.82246703342411321825e-2_f64) * t121302 + F::cast_from(0.41123351671205660912e-2_f64) * t121305 - F::cast_from(0.82246703342411321825e-2_f64) * t121308 - F::cast_from(0.16449340668482264365e-1_f64) * t121311 - F::cast_from(0.16449340668482264365e-1_f64) * t121314 - F::cast_from(0.82246703342411321825e-2_f64) * t121318;
    t121320
}
