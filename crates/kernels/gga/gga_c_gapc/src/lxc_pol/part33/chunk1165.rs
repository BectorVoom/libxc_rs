//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 1165/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk1165<F: Float>(t12744: F, t5407: F, t9113: F, t1643: F, t22327: F, t3679: F, t1266: F, t205: F, t3683: F, t34410: F, t34413: F, t34417: F, t34421: F, t34424: F, t34426: F, t34428: F, t34433: F) -> F {
    let t34436 = t9113 * t12744 * t5407;
    let t34439 = t1643 * t3679 * t22327;
    let t34442 = t1266 * t3683 * t205;
    let t34444 = F::cast_from(0.33148893438893365995e-7_f64) * t34410 + F::cast_from(0.49166375783284505216e-8_f64) * t34413 - F::cast_from(0.44524025454273061491e-5_f64) * t34417 - F::cast_from(0.24458523220486111112e-4_f64) * t34421 + F::cast_from(0.12501199801949976838e-2_f64) * t34424 + F::cast_from(0.16217772716043213195e-2_f64) * t34426 + F::cast_from(0.16217772716043213195e-2_f64) * t34428 - F::cast_from(0.2209926229259557733e-7_f64) * t34433 - F::cast_from(0.75033745761086241293e-8_f64) * t34436 + F::cast_from(0.10860115658064651693e-4_f64) * t34439 - F::cast_from(0.11594181388521408695e-4_f64) * t34442;
    t34444
}
