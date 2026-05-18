//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 477/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk477<F: Float>(t2749: F, t875: F, t296: F, t304: F, t305: F, t856: F, t91: F, t1771: F, t303: F, t1775: F, t849: F, t458: F, t854: F) -> (F, F, F, F, F, F, F, F) {
    let t2750 = t2749 * t875;
    let t2751 = t296 * t2750;
    let t2755 = F::new(1.0) / t305 / t304;
    let t2756 = t856 * t856;
    let t2758 = t91 * t2755 * t2756;
    let t2761 = F::new(4.0) / F::new(9.0) * t1771 * t303;
    let t2762 = t1775 * t849;
    let t2764 = t458 * t854;
    (t2750, t2751, t2755, t2756, t2758, t2761, t2762, t2764)
}
