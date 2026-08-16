//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 823/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk823(t17859: f64, t7380: f64, t5542: f64, t8687: f64, t674: f64, t2007: f64, t1970: f64, t1971: f64, t236: f64, t27724: f64, t38412: f64, t38415: f64, t38420: f64, t38426: f64, t38428: f64, t38432: f64, t38436: f64, t38442: f64, t38448: f64, t38450: f64, t38457: f64, t38460: f64, t38465: f64, t38467: f64) -> (f64, f64, f64) {
    let t38469 = t17859 * t7380;
    let t38471 = t8687 * t5542;
    let t38472 = t38471 * t674;
    let t38473 = t38472 * t2007;
    let t38477 = t1970 * t1971 * t236 * t27724;
    let t38479 = 0.42564599893297839398e-5_f64 * t38412 + t38415 + 0.76616279807936110914e-4_f64 * t38420 + 0.25538759935978703638e-4_f64 * t38426 - 0.25538759935978703638e-4_f64 * t38428 - 0.25538759935978703638e-4_f64 * t38432 + 0.25538759935978703638e-4_f64 * t38436 - 0.1064114997332445985e-4_f64 * t38442 - 0.25538759935978703638e-4_f64 * t38448 + 0.25538759935978703638e-4_f64 * t38450 - 0.38906704589967556326e-4_f64 * t38457 - 0.55866037359953414211e-4_f64 * t38460 - 0.31923449919973379548e-4_f64 * t38465 - 0.76616279807936110914e-4_f64 * t38467 - 0.25538759935978703638e-4_f64 * t38469 + 0.25538759935978703638e-4_f64 * t38473 - 0.42564599893297839398e-5_f64 * t38477;
    (t38471, t38472, t38479)
}
