//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 685/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk685<F: Float>(t11530: F, t445: F, t1849: F, t213: F, t4597: F, t967: F, t10487: F, t167: F, t11458: F, t1049: F, t695: F, t642: F) -> (F, F, F, F, F, F, F) {
    let t11532 = F::new(0.72818958333333333333e-4) * t445 * t11530;
    let t11612 = t213 * t1849;
    let t11625 = t967 * t4597;
    let t11630 = t167 * t10487;
    let t11633 = F::new(0.71734315950379065738e-1) * t11458;
    let t11634 = t1049 * t695;
    let t11635 = F::new(0.62154466893555682512e-3) * t11634;
    let t11682 = t642 * t1849;
    (t11532, t11612, t11625, t11630, t11633, t11635, t11682)
}
