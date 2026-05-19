//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 823/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk823<F: Float>(t17859: F, t7380: F, t5542: F, t8687: F, t674: F, t2007: F, t1970: F, t1971: F, t236: F, t27724: F, t38412: F, t38415: F, t38420: F, t38426: F, t38428: F, t38432: F, t38436: F, t38442: F, t38448: F, t38450: F, t38457: F, t38460: F, t38465: F, t38467: F) -> (F, F, F) {
    let t38469 = t17859 * t7380;
    let t38471 = t8687 * t5542;
    let t38472 = t38471 * t674;
    let t38473 = t38472 * t2007;
    let t38477 = t1970 * t1971 * t236 * t27724;
    let t38479 = F::cast_from(0.42564599893297839398e-5_f64) * t38412 + t38415 + F::cast_from(0.76616279807936110914e-4_f64) * t38420 + F::cast_from(0.25538759935978703638e-4_f64) * t38426 - F::cast_from(0.25538759935978703638e-4_f64) * t38428 - F::cast_from(0.25538759935978703638e-4_f64) * t38432 + F::cast_from(0.25538759935978703638e-4_f64) * t38436 - F::cast_from(0.1064114997332445985e-4_f64) * t38442 - F::cast_from(0.25538759935978703638e-4_f64) * t38448 + F::cast_from(0.25538759935978703638e-4_f64) * t38450 - F::cast_from(0.38906704589967556326e-4_f64) * t38457 - F::cast_from(0.55866037359953414211e-4_f64) * t38460 - F::cast_from(0.31923449919973379548e-4_f64) * t38465 - F::cast_from(0.76616279807936110914e-4_f64) * t38467 - F::cast_from(0.25538759935978703638e-4_f64) * t38469 + F::cast_from(0.25538759935978703638e-4_f64) * t38473 - F::cast_from(0.42564599893297839398e-5_f64) * t38477;
    (t38471, t38472, t38479)
}
