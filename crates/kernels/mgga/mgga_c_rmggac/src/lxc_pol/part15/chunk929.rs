//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 929/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk929<F: Float>(t1525: F, t1971: F, t511: F, t558: F, t7230: F, t1737: F, t495: F, t880: F, t10018: F, t7244: F, t1916: F, t2150: F, t2868: F, t41774: F, t41790: F, t41792: F, t41812: F, t41813: F, t47478: F, t47484: F, t47487: F, t47490: F, t47493: F, t47495: F, t47500: F, t8988: F) -> (F,) {
    let t47505 = t7230 * t1971 * t511 * t558 * t1525;
    let t47510 = t7230 * t1971 * t880 * t1737 * t495;
    let t47512 = t7244 * t10018;
    let t47515 = -0.19863479950205658386e-3 * t41774 - 0.72732431077987577941e-1 * t47478 - 0.11974241701863808564e0 * t2868 * t8988 + t41790 + t41792 - 0.19957069503106347607e-1 * t1916 * t2150 - 0.19863479950205658386e-4 * t47484 + 0.8980681276397856423e-1 * t47487 - 0.17961362552795712846e0 * t47490 - 0.44903406381989282115e-1 * t47493 + 0.31923449919973379548e-4 * t47495 + 0.31923449919973379548e-4 * t47500 + 0.31923449919973379548e-4 * t47505 - 0.63846899839946759095e-4 * t47510 + 0.99317399751028291929e-5 * t47512 - t41812 + 0.59590439850616975157e-4 * t41813;
    (t47515,)
}
