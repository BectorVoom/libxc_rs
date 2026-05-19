//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 881/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk881<F: Float>(t45743: F, t10914: F, t10915: F, t45369: F, t45316: F, t7584: F, t7585: F, t11765: F, t9823: F, t2536: F, t3614: F, t2009: F, t2021: F) -> (F, F, F, F, F) {
    let t45744 = F::cast_from(0.19171462976960374838e0_f64) * t45743;
    let t45747 = F::cast_from(0.21450293971110256001e2_f64) * t10914 * t10915 * t45369;
    let t45753 = F::cast_from(0.43710935587469654631e2_f64) * t7584 * t7585 * t45316;
    let t45755 = F::cast_from(0.35750489951850426669e0_f64) * t9823 * t11765;
    let t45758 = t2536 * t3614;
    let t45761 = F::cast_from(0.35750489951850426669e0_f64) * t2021 * t45758 * t2009;
    (t45744, t45747, t45753, t45755, t45761)
}
