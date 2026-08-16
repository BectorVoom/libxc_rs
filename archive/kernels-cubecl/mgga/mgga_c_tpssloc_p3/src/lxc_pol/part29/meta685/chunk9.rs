//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2347/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2347<F: Float>(t2320: F, t8103: F, t91708: F, t91713: F, t91715: F, t91718: F, t91722: F, t91724: F, t91726: F, t91730: F, t91735: F, t91737: F, t91739: F, t91747: F, t91749: F, t91752: F, t91755: F, t91757: F, t91759: F, t91762: F) -> F {
    let t96232 = -F::cast_from(2.0_f64) * t2320 * t8103 - t91708 - t91713 - t91715 - t91718 - t91722 - t91724 - t91726 - t91730 - t91735 - t91737 - t91739 - t91747 - t91749 - t91752 - t91755 - t91757 - t91759 - t91762;
    t96232
}
