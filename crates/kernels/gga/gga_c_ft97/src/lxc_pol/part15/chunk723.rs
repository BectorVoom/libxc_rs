//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 723/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk723<F: Float>(t139: F, t20630: F, t527: F, t1013: F, t4702: F, t8908: F, t133: F, t11299: F, t15840: F, t15855: F, t15866: F, t20067: F, t20071: F, t20074: F, t20078: F, t20081: F, t20085: F, t8914: F) -> (F, F, F, F, F) {
    let t20631 = t139 * t20630;
    let t20632 = t527 * t20631;
    let t20634 = t4702 * t1013;
    let t20635 = t8908 * t20634;
    let t20636 = t133 * t20635;
    let t20651 = t8914 - F::new(0.11113000182098765433e-1) * t11299 + F::new(0.22226000364197530866e-1) * t15840 - F::new(0.33339000546296296299e-1) * t15855 + F::new(0.16669500273148148149e-1) * t15866 + F::new(0.51860667516460905352e-1) * t20067 - F::new(0.13335600218518518519e0) * t20071 + F::new(0.66678001092592592595e-1) * t20074 + F::new(0.10001700163888888889e0) * t20078 - F::new(0.10001700163888888889e0) * t20081 + F::new(0.16669500273148148149e-1) * t20085;
    (t20631, t20632, t20634, t20636, t20651)
}
