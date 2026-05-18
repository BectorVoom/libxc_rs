//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 878/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk878<F: Float>(t11622: F, t2464: F, t2465: F, t825: F, t13641: F, t2013: F, t45466: F, t969: F, t32809: F, t32810: F, t45369: F, t11801: F, t2624: F, t4752: F) -> (F, F, F, F, F) {
    let t45687 = t825 * t2464 * t2465 * t11622;
    let t45688 = F::new(0.42603251059911944084e-1) * t45687;
    let t45689 = t2013 * t13641;
    let t45690 = F::new(0.19171462976960374838e0) * t45689;
    let t45692 = t825 * t969 * t45466;
    let t45693 = F::new(0.19171462976960374838e0) * t45692;
    let t45700 = F::new(0.85801175884441024004e1) * t32809 * t32810 * t45369;
    let t45703 = F::new(0.28600391961480341335e1) * t11801 * t4752 * t2624;
    (t45688, t45690, t45693, t45700, t45703)
}
