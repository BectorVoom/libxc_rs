//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 916/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk916<F: Float>(t1590: F, t609: F, t4313: F, t622: F, t1588: F, t4413: F, t12305: F, t1625: F, t4479: F, t1627: F, t629: F, t2791: F, t838: F, t169: F, t2628: F, t174: F, t2640: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12856 = t1590 * t1590;
    let t12857 = 1.0 / t12856;
    let t12858 = t609 * t12857;
    let t12861 = 1.0 / t4313 / t622;
    let t12890 = t1588 * t4413;
    let t12915 = 0.51588271604938271604e-3 * t12305;
    let t12933 = t1625 * t4479;
    let t12938 = t1627 * t1627;
    let t12939 = 1.0 / t12938;
    let t12940 = t629 * t12939;
    let t13000 = t838 * t2791;
    let t13003 = 1.0 / t2628 / t169;
    let t13014 = 1.0 / t2640 / t174;
    (t12858, t12861, t12890, t12915, t12933, t12940, t13000, t13003, t13014)
}
