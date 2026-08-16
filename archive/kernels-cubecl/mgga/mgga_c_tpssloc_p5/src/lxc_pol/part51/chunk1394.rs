//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1394/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1394<F: Float>(t112990: F, t112995: F, t114659: F, t114666: F, t118725: F, t118728: F, t118730: F, t118735: F, t118736: F, t118737: F, t121517: F, t121521: F, t121524: F, t121528: F, t1499: F, t31397: F, t33396: F, t808: F) -> F {
    let t121531 = t1499 * t31397 + F::cast_from(0.38381794893125283518e-1_f64) * t114659 + F::cast_from(0.82246703342411321824e-2_f64) * t114666 - F::cast_from(0.16449340668482264365e-1_f64) * t121517 - F::cast_from(0.16449340668482264365e-1_f64) * t121521 - t118725 + t118728 + F::cast_from(0.41123351671205660912e-2_f64) * t121524 - F::cast_from(0.82246703342411321825e-2_f64) * t121528 + t118730 - t118735 + t808 * t33396 + t112990 - t118736 + t112995 - t118737;
    t121531
}
