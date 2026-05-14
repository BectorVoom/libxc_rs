//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1274/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1274<F: Float>(t10921: F, t2490: F, t3496: F, t9135: F, t10914: F, t2496: F, t952: F, t10927: F, t3490: F, t9151: F, t9154: F, t2213: F, t238: F, t4265: F, t10937: F, t801: F) -> (F, F, F, F, F, F, F, F) {
    let t29855 = t10921 * t2490;
    let t29857 = t3496 * t9135;
    let t29860 = t2496 * t10914 * t952;
    let t29862 = t10927 * t2490;
    let t29864 = t952 * t3490;
    let t29865 = t9151 * t29864;
    let t29867 = t9154 * t29864;
    let t29870 = t238 * t2213 * t4265;
    let t29873 = t238 * t801 * t10937;
    (t29855, t29857, t29860, t29862, t29865, t29867, t29870, t29873)
}
