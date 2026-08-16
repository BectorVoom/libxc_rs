//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1163/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1163<F: Float>(t34372: F, t3714: F, t34344: F, t34346: F, t34351: F, t34353: F, t34356: F, t34359: F, t34361: F, t34364: F, t34367: F, t34370: F) -> F {
    let t34373 = t34372 * t3714;
    let t34375 = F::cast_from(0.57970906942607043474e-5_f64) * t34344 + F::cast_from(0.21720231316129303386e-4_f64) * t34346 - F::cast_from(0.25340269868817520618e-3_f64) * t34351 - F::cast_from(0.20241536458333333334e-4_f64) * t34353 + F::cast_from(0.28960308421505737848e-5_f64) * t34356 + F::cast_from(0.28960308421505737848e-5_f64) * t34359 - F::cast_from(0.2845640240200497334e-7_f64) * t34361 + F::cast_from(0.50595483470764842601e-7_f64) * t34364 + F::cast_from(0.11594181388521408695e-4_f64) * t34367 - F::cast_from(0.2318836277704281739e-4_f64) * t34370 + F::cast_from(0.34180192345881159604e-5_f64) * t34373;
    t34375
}
