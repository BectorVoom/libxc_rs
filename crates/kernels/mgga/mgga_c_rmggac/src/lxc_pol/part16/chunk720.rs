//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 720/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk720<F: Float>(t10492: F, t10420: F, t884: F, t10085: F, t10091: F, t10096: F, t10098: F, t10103: F, t10107: F, t1916: F, t708: F, t10113: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10493 = F::new(0.11974241701863808564e0) * t10492;
    let t10494 = t884 * t10420;
    let t10495 = F::new(0.59871208509319042821e-1) * t10494;
    let t10497 = F::new(0.5107751987195740728e-4) * t10085;
    let t10498 = F::new(0.5107751987195740728e-4) * t10091;
    let t10499 = F::new(0.1702583995731913576e-4) * t10096;
    let t10500 = F::new(0.1702583995731913576e-4) * t10098;
    let t10501 = F::new(0.638468998399467591e-4) * t10103;
    let t10502 = F::new(0.15323255961587222184e-3) * t10107;
    let t10505 = t1916 * t708;
    let t10506 = F::new(0.19957069503106347607e-1) * t10505;
    let t10507 = F::new(0.5987120850931904282e-1) * t10113;
    (t10493, t10495, t10497, t10498, t10499, t10500, t10501, t10502, t10506, t10507)
}
