//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1258/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1258<F: Float>(t8124: F, t25836: F, t2602: F, t7274: F, t930: F, t2704: F, t7882: F, t11518: F, t19: F, t23951: F, t24569: F, t25087: F, t25595: F, t2596: F, t25969: F, t25970: F, t25972: F, t25975: F, t25977: F, t25979: F, t2643: F, t2668: F, t2708: F, t323: F, t7867: F, t8058: F, t894: F, t953: F, sigma0: F) -> F {
    let t25981 = t8124 * sigma0;
    let t25982 = t25981 * t25836;
    let t25996 = t930 * t7274 * t2602;
    let t26000 = t2704 * t7882;
    let t26004 = -F::new(0.1209136907000740735e0) * t953 * t25087 - t25969 - F::new(0.93770531639908660928e4) * t25970 + F::new(0.46885265819954330464e4) * t25972 + F::new(0.16829779668897917239e1) * t25975 + F::new(0.80782942410710002747e1) * t25977 - F::new(0.12117441361606500412e2) * t25979 + F::new(0.18014732272771396904e7) * t25982 * t323 * t24569 * t19 + F::new(0.33587136305576131525e-1) * t953 * t894 * t2596 * t23951 + F::new(0.61944912485988186947e2) * t2668 * t11518 * t25595 * t2643 + F::new(0.7727254657590006982e-1) * t25996 - F::new(0.12475836244235246496e3) * t2708 * t8058 + F::new(0.42991534471137448352e0) * t26000 - F::new(0.64487301706706172529e0) * t2704 * t7867;
    t26004
}
