//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1381/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1381<F: Float>(t1162: F, t12601: F, t12602: F, t12621: F, t27038: F, t27346: F, t27374: F, t27378: F, t27422: F, t27425: F, t27438: F, t27439: F, t27443: F, t27449: F, t27455: F, t27461: F, t27465: F, t27470: F, t27473: F, t27483: F, t3103: F, t3234: F, t3235: F, t3244: F, t3245: F, t4289: F, t4387: F, t4435: F, t8455: F, t8469: F, t8482: F, t8532: F, t8907: F, t8915: F, t9062: F, t914: F) -> F {
    let t27492 = F::new(0.7727254657590006982e-1) * t27422 - F::new(0.18583473745796456084e3) * t4435 * t12602 * t27425 + F::new(0.3029360340401625103e1) * t3244 * t4289 * t27378 - F::new(0.30972456242994093474e2) * t3103 * t9062 * t8455 + F::new(0.22720202553012188272e1) * t3244 * t3245 * t27374 + F::new(0.31957282085435444036e5) * t27438 * t27439 * t8915 * t27443 + F::new(0.779739765264702906e2) * t12601 * t12621 * t27449 - F::new(0.41296608323992124631e2) * t27455 - F::new(0.13909058383662012568e1) * t1162 * t914 * t8532 * t27346 - F::new(0.12117441361606500412e2) * t3244 * t4289 * t27461 - F::new(0.4678438591588217436e2) * t3234 * t3235 * t27465 + F::new(0.12388982497197637389e3) * t27470 - F::new(0.80609127133382715662e-1) * t27473 - F::new(0.1559479530529405812e3) * t3234 * t4387 * t27461 - F::new(0.10818156520626009775e1) * t1162 * t914 * t27038 - F::new(0.23967961564076583027e5) * t27483 - F::new(0.30972456242994093473e2) * t3103 * t8469 * t8907 + F::new(0.54090782603130048873e0) * t1162 * t914 * t8482 * t27346;
    t27492
}
