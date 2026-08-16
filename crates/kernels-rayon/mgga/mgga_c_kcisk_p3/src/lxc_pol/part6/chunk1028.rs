//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1028/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1028(t2191: f64, t7877: f64, t14140: f64, t5658: f64, t7897: f64, t1349: f64, t14093: f64, t2110: f64, t2209: f64, t30738: f64, t30875: f64, t30877: f64, t30880: f64, t30883: f64, t30886: f64, t30889: f64, t338: f64, t3819: f64, t417: f64, t451: f64, t7828: f64, t8159: f64) -> (f64, f64) {
    let t30892 = t7877 * t2191;
    let t30893 = t14140 * t30892;
    let t30896 = t5658 * t7897;
    let t30899 = -t30738 * t451 - 3.0_f64 * t7828 * t2209 - 3.0_f64 * t2110 * t8159 - t338 * t30875 - 0.14055920378328537299e-1_f64 * t14093 * t30877 - 0.28111840756657074597e-1_f64 * t3819 * t30880 + 0.14055920378328537299e-1_f64 * t3819 * t30883 + 0.14055920378328537299e-1_f64 * t1349 * t30886 + 0.14055920378328537299e-1_f64 * t1349 * t30889 - 0.56223681513314149196e-1_f64 * t417 * t30893 + 0.42167761134985611897e-1_f64 * t417 * t30896;
    (t30892, t30899)
}
