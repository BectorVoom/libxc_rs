//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 1197/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk1197(t3076: f64, t34714: f64, t11455: f64, t1453: f64, t505: f64, t5526: f64, t674: f64, t34808: f64, t34811: f64, t34813: f64, t34820: f64, t34822: f64, t34824: f64, t34826: f64, t34830: f64, t34832: f64) -> f64 {
    let t34834 = t34714 * t3076;
    let t34839 = t11455 * t1453 * t505 * t674 * t5526;
    let t34841 = 0.38647271295071362318e-6_f64 * t34808 - 0.687148483626368822e-6_f64 * t34811 + 0.13717106646948578487e-6_f64 * t34813 - 0.10456390683076999807e-9_f64 * t34820 - 0.1800809898266069791e-6_f64 * t34822 + 0.42233783114695867695e-6_f64 * t34824 - 0.25301920572916666668e-5_f64 * t34826 + 0.21720231316129303386e-4_f64 * t34830 - 0.40022999988963401106e-7_f64 * t34832 + 0.32018399991170720886e-6_f64 * t34834 + 0.10110318318802209383e-5_f64 * t34839;
    t34841
}
