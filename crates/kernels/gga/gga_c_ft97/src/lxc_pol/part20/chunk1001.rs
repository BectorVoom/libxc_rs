//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1001/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1001<F: Float>(t2387: F, t96599: F, t2378: F, t420: F, t703: F, t24322: F, t3771: F, t5567: F, t13522: F, t24265: F, t697: F, t24305: F, t27669: F, t229: F, t9: F, t6043: F, t6046: F, t96535: F) -> (F, F, F, F, F, F, F) {
    let t96600 = t2387 * t96599;
    let t96602 = t420 * t703 * t2378;
    let t96607 = t3771 * t24322 * t5567;
    let t96612 = t24265 * t697 * t13522;
    let t96614 = t24305 * t27669;
    let t96615 = t9 * t229;
    let t96623 = t6043 * t96535 * t6046;
    (t96600, t96602, t96607, t96612, t96614, t96615, t96623)
}
