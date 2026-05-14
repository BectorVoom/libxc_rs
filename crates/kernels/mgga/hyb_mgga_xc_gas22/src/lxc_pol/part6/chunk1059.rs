//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1059/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1059<F: Float>(t2496: F, t4251: F, t952: F, t10914: F, t957: F, t238: F, t4261: F, t801: F, t1392: F, t3470: F, t242: F, t4265: F, t4234: F, t940: F, t10911: F, t343: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10927 = t2496 * t4251;
    let t10928 = t10927 * t952;
    let t10930 = t957 * t10914;
    let t10935 = t238 * t801 * t4261;
    let t10937 = t1392 * t3470;
    let t10939 = t238 * t242 * t10937;
    let t10942 = t238 * t801 * t4265;
    let t10944 = t940 * t4234;
    let t10946 = t238 * t242 * t10944;
    let t10948 = t343 * t10911;
    (t10927, t10928, t10930, t10935, t10937, t10939, t10942, t10944, t10946, t10948)
}
