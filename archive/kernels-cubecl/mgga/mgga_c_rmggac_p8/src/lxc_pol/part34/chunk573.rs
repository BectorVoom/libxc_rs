//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 573/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk573<F: Float>(t14635: F, t2191: F, t3219: F, t1986: F, t2229: F, t675: F, t2186: F, t14119: F, t14128: F, t14133: F, t14144: F, t1356: F, t14441: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t14636 = F::cast_from(0.39914139006212695214e-1_f64) * t14635;
    let t14637 = t2191 * t3219;
    let t14638 = F::cast_from(0.42564599893297839398e-5_f64) * t14637;
    let t14639 = t1986 * t2229;
    let t14640 = t675 * t14639;
    let t14641 = F::cast_from(0.42564599893297839398e-5_f64) * t14640;
    let t14642 = t2186 * t3219;
    let t14644 = F::cast_from(0.17519306092901367187e-5_f64) * t14119;
    let t14645 = F::cast_from(0.35038612185802734376e-6_f64) * t14128;
    let t14646 = F::cast_from(0.35038612185802734376e-6_f64) * t14133;
    let t14649 = F::cast_from(0.14967802127329760705e-1_f64) * t14144;
    let t14650 = t1356 * t14441;
    (t14636, t14638, t14639, t14641, t14642, t14644, t14645, t14646, t14649, t14650)
}
