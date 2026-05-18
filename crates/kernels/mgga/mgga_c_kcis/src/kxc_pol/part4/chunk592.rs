//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 592/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk592<F: Float>(t1065: F, t738: F, t1064: F, t2829: F, t2845: F, t945: F, t1080: F, t743: F, t2850: F, t104: F, t111: F, t3105: F, t3109: F, t3113: F, t3114: F, t3116: F, t3119: F) -> (F, F, F, F, F, F) {
    let t3122 = t738 * t1065;
    let t3124 = t1064 * t2829;
    let t3127 = t945 * t2845;
    let t3130 = t743 * t1080;
    let t3132 = t1064 * t2850;
    let t3135 = t3105 - t3109 - t3113 + F::new(0.9368e-2) * t3114 - F::new(0.3513e-2) * t104 * t3116 + F::new(0.1171e-2) * t104 * t3119 - F::new(0.26416666666666666666e-2) * t3122 + F::new(0.7925e-3) * t111 * t3124 - F::new(0.52833333333333333333e-3) * t111 * t3127 - F::new(0.23526125e-4) * t3130 - F::new(0.1585e-2) * t111 * t3132;
    (t3122, t3124, t3127, t3130, t3132, t3135)
}
