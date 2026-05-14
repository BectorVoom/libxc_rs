//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1075/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1075<F: Float>(t27036: F, t27043: F, t35139: F, t11408: F, t561: F, t8951: F, t11413: F, t8960: F, t19546: F, t33623: F, t5462: F, t35334: F, t35336: F, t35339: F, t35341: F, t35343: F, t35346: F, t35349: F) -> (F,) {
    let t35352 = t27036 * t35139 * t27043;
    let t35355 = t561 * t11408 * t8951;
    let t35358 = t561 * t11413 * t8960;
    let t35361 = t5462 * t33623 * t19546;
    let t35363 = -0.57970906942607043474e-5 * t35334 + 0.11594181388521408695e-4 * t35336 + 0.57970906942607043474e-5 * t35339 + 0.18115908419564701086e-6 * t35341 - 0.18115908419564701086e-6 * t35343 - 0.14340192936791314021e-8 * t35346 + 0.9560128624527542681e-9 * t35349 - 0.61551119569641057312e-8 * t35352 + 0.8433973524305555556e-6 * t35355 - 0.73797268337673611116e-6 * t35358 - 0.10110318318802209383e-5 * t35361;
    (t35363,)
}
