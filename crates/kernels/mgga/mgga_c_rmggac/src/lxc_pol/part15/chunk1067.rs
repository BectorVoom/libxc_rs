//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1067/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1067<F: Float>(t10018: F, t7244: F, t1916: F, t2150: F, t2868: F, t41774: F, t41790: F, t41792: F, t41812: F, t41813: F, t47478: F, t47484: F, t47487: F, t47490: F, t47493: F, t47495: F, t47500: F, t47505: F, t47510: F, t8988: F) -> F {
    let t47512 = t7244 * t10018;
    let t47515 = -F::new(0.19863479950205658386e-3) * t41774 - F::new(0.72732431077987577941e-1) * t47478 - F::new(0.11974241701863808564e0) * t2868 * t8988 + t41790 + t41792 - F::new(0.19957069503106347607e-1) * t1916 * t2150 - F::new(0.19863479950205658386e-4) * t47484 + F::new(0.8980681276397856423e-1) * t47487 - F::new(0.17961362552795712846e0) * t47490 - F::new(0.44903406381989282115e-1) * t47493 + F::new(0.31923449919973379548e-4) * t47495 + F::new(0.31923449919973379548e-4) * t47500 + F::new(0.31923449919973379548e-4) * t47505 - F::new(0.63846899839946759095e-4) * t47510 + F::new(0.99317399751028291929e-5) * t47512 - t41812 + F::new(0.59590439850616975157e-4) * t41813;
    t47515
}
