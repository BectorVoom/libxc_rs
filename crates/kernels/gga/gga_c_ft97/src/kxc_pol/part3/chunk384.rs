//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 384/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk384<F: Float>(t342: F, t630: F, t784: F, t294: F, t668: F, t505: F, t231: F, t824: F, t1526: F, t2320: F, t343: F, t830: F, t10: F, t1542: F, t296: F, t2336: F, t793: F, t89: F) -> (F, F, F, F, F, F, F, F) {
    let t2638 = t342 * t630 * t784 / 12.0;
    let t2639 = t294 * t668;
    let t2640 = t2639 * t505;
    let t2644 = t231 * t824;
    let t2648 = t830 - t2638 - t1526 * t2320 * t2640 / 12.0 - t342 * t343 * t2644 / 4.0;
    let t2652 = t10 * t1542 * t296;
    let t2653 = 2.0 / 27.0 * t2652;
    let t2655 = t89 * t2336 * t793;
    (t2638, t2639, t2640, t2644, t2648, t2652, t2653, t2655)
}
