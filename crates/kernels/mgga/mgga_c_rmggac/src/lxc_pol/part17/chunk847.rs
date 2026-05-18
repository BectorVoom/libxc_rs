//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 847/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk847<F: Float>(t41817: F, t388: F, t535: F, t7933: F, t7934: F, t7244: F, t8422: F, t2310: F, t7939: F, t2283: F, t38354: F, t7473: F) -> (F, F, F, F, F, F) {
    let t41818 = F::new(0.72042316457491791906e-3) * t41817;
    let t41821 = t7933 * t7934 * t388 * t535;
    let t41822 = F::new(0.72042316457491791906e-3) * t41821;
    let t41828 = t7244 * t8422;
    let t41829 = F::new(0.19863479950205658386e-4) * t41828;
    let t41882 = t7939 * t2310;
    let t41883 = F::new(0.19863479950205658386e-4) * t41882;
    let t41884 = t7939 * t2283;
    let t41885 = F::new(0.19863479950205658386e-4) * t41884;
    let t41890 = t38354 * t7473;
    (t41818, t41822, t41829, t41883, t41885, t41890)
}
