//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1015/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1015<F: Float>(t34410: F, t34413: F, t34417: F, t34421: F, t34424: F, t34426: F, t34428: F, t34433: F, t34436: F, t34439: F, t34442: F, t144: F, t3095: F, t3094: F, t3954: F, t128: F, t3141: F, t33655: F, t5462: F, t623: F) -> (F, F, F, F) {
    let t34444 = 0.33148893438893365995e-7 * t34410 + 0.49166375783284505216e-8 * t34413 - 0.44524025454273061491e-5 * t34417 - 0.24458523220486111112e-4 * t34421 + 0.12501199801949976838e-2 * t34424 + 0.16217772716043213195e-2 * t34426 + 0.16217772716043213195e-2 * t34428 - 0.2209926229259557733e-7 * t34433 - 0.75033745761086241293e-8 * t34436 + 0.10860115658064651693e-4 * t34439 - 0.11594181388521408695e-4 * t34442;
    let t34447 = t3095 * t144;
    let t34449 = t3094 * t34447 * t3954;
    let t34454 = t5462 * t33655 * t3141 * t623 * t128;
    (t34444, t34447, t34449, t34454)
}
