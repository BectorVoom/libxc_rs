//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 591/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk591<F: Float>(t1526: F, t1527: F, t1953: F, t1970: F, t2081: F, t3088: F, t342: F, t343: F, t8759: F, t8761: F, t8764: F, t8767: F, t8775: F, t8779: F, t8783: F, t520: F, t7773: F, t89: F) -> (F, F) {
    let t8787 = t1953 + t2081 + t8759 - t8761 / 18.0 - t8764 / 6.0 - t1526 * t3088 * t8767 / 9.0 - t1526 * t1527 * t1970 / 6.0 + t1526 * t1527 * t8775 / 6.0 - t1526 * t1527 * t8779 / 12.0 - t342 * t343 * t8783 / 4.0;
    let t8796 = t89 * t7773 * t520;
    (t8787, t8796)
}
