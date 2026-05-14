//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1042/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1042<F: Float>(t11474: F, t8880: F, t3076: F, t34714: F, t11455: F, t1453: F, t505: F, t5526: F, t674: F, t34808: F, t34811: F, t34813: F, t34820: F, t34822: F, t34824: F, t34826: F, t34830: F) -> (F,) {
    let t34832 = t11474 * t8880;
    let t34834 = t34714 * t3076;
    let t34839 = t11455 * t1453 * t505 * t674 * t5526;
    let t34841 = 0.38647271295071362318e-6 * t34808 - 0.687148483626368822e-6 * t34811 + 0.13717106646948578487e-6 * t34813 - 0.10456390683076999807e-9 * t34820 - 0.1800809898266069791e-6 * t34822 + 0.42233783114695867695e-6 * t34824 - 0.25301920572916666668e-5 * t34826 + 0.21720231316129303386e-4 * t34830 - 0.40022999988963401106e-7 * t34832 + 0.32018399991170720886e-6 * t34834 + 0.10110318318802209383e-5 * t34839;
    (t34841,)
}
