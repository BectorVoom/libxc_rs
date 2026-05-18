//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1106/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1106<F: Float>(t132: F, t10850: F, t338: F, t4222: F, t930: F, t10325: F, t2598: F, t4323: F, t3605: F, t3604: F, t9057: F, t4310: F, t6992: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t133 = t132 <= zeta_threshold;
    let t10851 = t10850 * t338;
    let t10852 = t4222 * t930;
    let t10853 = piecewise3::<f64>(t133, F::new(0.0), t10325);
    let t10864 = t2598 * t4323;
    let t10865 = t10864 * t3605;
    let t10868 = t3604 * t9057;
    let t10871 = t6992 * t4310;
    (t10851, t10852, t10853, t10864, t10865, t10868, t10871)
}
