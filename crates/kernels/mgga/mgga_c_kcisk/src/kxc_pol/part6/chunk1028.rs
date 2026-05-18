//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1028/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1028<F: Float>(t2191: F, t7877: F, t14140: F, t5658: F, t7897: F, t1349: F, t14093: F, t2110: F, t2209: F, t30738: F, t30875: F, t30877: F, t30880: F, t30883: F, t30886: F, t30889: F, t338: F, t3819: F, t417: F, t451: F, t7828: F, t8159: F) -> (F, F) {
    let t30892 = t7877 * t2191;
    let t30893 = t14140 * t30892;
    let t30896 = t5658 * t7897;
    let t30899 = -t30738 * t451 - F::new(3.0) * t7828 * t2209 - F::new(3.0) * t2110 * t8159 - t338 * t30875 - F::new(0.14055920378328537299e-1) * t14093 * t30877 - F::new(0.28111840756657074597e-1) * t3819 * t30880 + F::new(0.14055920378328537299e-1) * t3819 * t30883 + F::new(0.14055920378328537299e-1) * t1349 * t30886 + F::new(0.14055920378328537299e-1) * t1349 * t30889 - F::new(0.56223681513314149196e-1) * t417 * t30893 + F::new(0.42167761134985611897e-1) * t417 * t30896;
    (t30892, t30899)
}
