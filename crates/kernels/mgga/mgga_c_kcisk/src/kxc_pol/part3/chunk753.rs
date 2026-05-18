//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 753/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk753<F: Float>(t1049: F, t695: F, t10399: F, t10441: F, t10449: F, t11495: F, t11613: F, t11615: F, t11623: F, t11626: F, t11630: F, t11633: F, t1809: F, t1850: F, t5089: F, t5168: F) -> F {
    let t11634 = t1049 * t695;
    let t11635 = F::new(0.62154466893555682512e-3) * t11634;
    let t11636 = F::new(0.11955719325063177623e-1) * t1809 * t10449 - F::new(0.93231700340333523768e-3) * t11613 + F::new(0.31077233446777841256e-3) * t11615 - F::new(0.5179538907796306876e-4) * t1850 * t10449 - F::new(0.71734315950379065738e-1) * t5089 * t10399 + F::new(0.46615850170166761884e-3) * t5168 * t10399 + F::new(0.71734315950379065738e-1) * t11623 - F::new(0.93231700340333523768e-3) * t11626 + F::new(0.71734315950379065738e-1) * t11495 * t10441 - F::new(0.62154466893555682512e-3) * t11630 * t10441 + t11633 - t11635;
    t11636
}
