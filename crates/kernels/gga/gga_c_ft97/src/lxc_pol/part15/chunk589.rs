//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 589/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk589<F: Float>(t9680: F, t226: F, t10: F, t242: F, t3050: F, t191: F, t7514: F, t27: F, t9567: F, t241: F, t9570: F, t2344: F, t375: F) -> (F, F, F, F, F, F, F, F) {
    let t9681 = F::new(1.0) / t9680;
    let t9682 = t226 * t9681;
    let t9698 = t10 * t3050 * t242;
    let t9699 = F::new(14.0) / F::new(81.0) * t9698;
    let t9707 = t191 * t7514;
    let t9716 = t27 * t9567;
    let t9717 = t241 * t9570;
    let t9725 = t375 * t2344;
    (t9681, t9682, t9698, t9699, t9707, t9716, t9717, t9725)
}
