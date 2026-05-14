//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 761/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk761<F: Float>(t13519: F, t3771: F, t200: F, t709: F, t3761: F, t3776: F, t236: F, t3750: F, t3724: F, t807: F, t9542: F, t1092: F, t1771: F, t3740: F, t458: F, t3743: F) -> (F, F, F, F, F, F, F, F) {
    let t13520 = t3771 * t13519;
    let t13521 = t200 * t709;
    let t13522 = t3761 * t13521;
    let t13523 = t3776 * t13522;
    let t13526 = t236 * t3750;
    let t13527 = t3724 * t13526;
    let t13531 = t807 * t9542;
    let t13538 = t1771 * t1092;
    let t13540 = t458 * t3740;
    let t13541 = 4.0 / 27.0 * t13540;
    let t13542 = t458 * t3743;
    (t13520, t13523, t13527, t13531, t13538, t13540, t13541, t13542)
}
