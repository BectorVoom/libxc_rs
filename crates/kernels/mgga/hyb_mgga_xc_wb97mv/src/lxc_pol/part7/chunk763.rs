//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 763/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk763<F: Float>(t4170: F, t788: F, t2217: F, t4166: F, t795: F, t1323: F, t238: F, t242: F, t226: F, t4153: F, t2214: F, t2227: F, t3317: F, t3359: F, t4155: F, t4167: F) -> (F, F, F, F, F, F, F, F) {
    let t4171 = t788 * t4170;
    let t4175 = t2217 * t4166;
    let t4177 = t795 * t4170;
    let t4180 = t1323 * t1323;
    let t4182 = t238 * t242 * t4180;
    let t4184 = t226 * t4153;
    let t4186 = t238 * t242 * t4184;
    let t4188 = -0.9494625e0 * t4167 + 0.1898925e1 * t4171 + t2214 - 0.59793333333333333334e0 * t3317 + 0.8969e0 * t4155 + 0.15358125e0 * t4175 + 0.3071625e0 * t4177 + t2227 - 0.32862666666666666666e0 * t3359 + 0.24647e0 * t4182 + 0.24647e0 * t4186;
    (t4171, t4175, t4177, t4180, t4182, t4184, t4186, t4188)
}
