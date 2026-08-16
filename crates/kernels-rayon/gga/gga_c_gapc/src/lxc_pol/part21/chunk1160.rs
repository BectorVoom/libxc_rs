//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1160/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1160(t34372: f64, t3714: f64, t34344: f64, t34346: f64, t34351: f64, t34353: f64, t34356: f64, t34359: f64, t34361: f64, t34364: f64, t34367: f64, t34370: f64) -> f64 {
    let t34373 = t34372 * t3714;
    let t34375 = 0.57970906942607043474e-5_f64 * t34344 + 0.21720231316129303386e-4_f64 * t34346 - 0.25340269868817520618e-3_f64 * t34351 - 0.20241536458333333334e-4_f64 * t34353 + 0.28960308421505737848e-5_f64 * t34356 + 0.28960308421505737848e-5_f64 * t34359 - 0.2845640240200497334e-7_f64 * t34361 + 0.50595483470764842601e-7_f64 * t34364 + 0.11594181388521408695e-4_f64 * t34367 - 0.2318836277704281739e-4_f64 * t34370 + 0.34180192345881159604e-5_f64 * t34373;
    t34375
}
