//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1010/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1010<F: Float>(t11577: F, t11580: F, t561: F, t21643: F, t26561: F, t1743: F, t26597: F, t21801: F, t5395: F, t5743: F, t5722: F, t1030: F, t33311: F, t3714: F, t34344: F, t34346: F, t34351: F, t34353: F, t34356: F) -> (F, F, F, F) {
    let t34359 = t561 * t11577 * t11580;
    let t34361 = t26561 * t21643;
    let t34363 = t1743 * t26597;
    let t34364 = t34363 * t21643;
    let t34366 = t5395 * t21801;
    let t34367 = t34366 * t5743;
    let t34370 = t1743 * t21801 * t5722;
    let t34372 = t1030 * t33311;
    let t34373 = t34372 * t3714;
    let t34375 = 0.57970906942607043474e-5 * t34344 + 0.21720231316129303386e-4 * t34346 - 0.25340269868817520618e-3 * t34351 - 0.20241536458333333334e-4 * t34353 + 0.28960308421505737848e-5 * t34356 + 0.28960308421505737848e-5 * t34359 - 0.2845640240200497334e-7 * t34361 + 0.50595483470764842601e-7 * t34364 + 0.11594181388521408695e-4 * t34367 - 0.2318836277704281739e-4 * t34370 + 0.34180192345881159604e-5 * t34373;
    (t34363, t34366, t34372, t34375)
}
