//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 741/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk741<F: Float>(t236: F, t615: F, t7230: F, t794: F, t9188: F, t17859: F, t7742: F, t7380: F, t5542: F, t8687: F, t674: F, t2007: F, t1970: F, t1971: F, t27724: F, t38412: F, t38415: F, t38420: F, t38426: F, t38428: F, t38432: F, t38436: F, t38442: F, t38448: F, t38450: F, t38457: F, t38460: F) -> (F, F, F) {
    let t38465 = t7230 * t9188 * t236 * t615 * t794;
    let t38467 = t17859 * t7742;
    let t38469 = t17859 * t7380;
    let t38471 = t8687 * t5542;
    let t38472 = t38471 * t674;
    let t38473 = t38472 * t2007;
    let t38477 = t1970 * t1971 * t236 * t27724;
    let t38479 = 0.42564599893297839398e-5 * t38412 + t38415 + 0.76616279807936110914e-4 * t38420 + 0.25538759935978703638e-4 * t38426 - 0.25538759935978703638e-4 * t38428 - 0.25538759935978703638e-4 * t38432 + 0.25538759935978703638e-4 * t38436 - 0.1064114997332445985e-4 * t38442 - 0.25538759935978703638e-4 * t38448 + 0.25538759935978703638e-4 * t38450 - 0.38906704589967556326e-4 * t38457 - 0.55866037359953414211e-4 * t38460 - 0.31923449919973379548e-4 * t38465 - 0.76616279807936110914e-4 * t38467 - 0.25538759935978703638e-4 * t38469 + 0.25538759935978703638e-4 * t38473 - 0.42564599893297839398e-5 * t38477;
    (t38471, t38472, t38479)
}
