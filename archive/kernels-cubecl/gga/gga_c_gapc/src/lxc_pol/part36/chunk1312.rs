//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1312/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1312<F: Float>(t34484: F, t34486: F, t34489: F, t34492: F, t34495: F, t34497: F, t34499: F, t34501: F, t34505: F, t34507: F, t34510: F, t34515: F, t34517: F, t34520: F, t34522: F, t34525: F, t34528: F, t34530: F, t34533: F, t34537: F, t34539: F, t34547: F) -> (F, F) {
    let t38168 = -F::cast_from(0.21524203265154803243e-6_f64) * t34484 + F::cast_from(0.26519114751114692796e-6_f64) * t34486 - F::cast_from(0.9275345110817126956e-4_f64) * t34489 + F::cast_from(0.9275345110817126956e-4_f64) * t34492 + F::cast_from(0.31433990684987949196e-7_f64) * t34495 - F::cast_from(0.84412963981222021456e-7_f64) * t34497 - F::cast_from(0.28137654660407340486e-7_f64) * t34499 - F::cast_from(0.25635144259410869702e-5_f64) * t34501 + F::cast_from(0.8839704917038230932e-7_f64) * t34505 - F::cast_from(0.25635144259410869702e-5_f64) * t34507 - F::cast_from(0.39333100626627604174e-6_f64) * t34510;
    let t38181 = F::cast_from(0.10120768229166666668e-4_f64) * t34515 + F::cast_from(0.50603841145833333338e-5_f64) * t34517 + F::cast_from(0.10120768229166666668e-4_f64) * t34520 + F::cast_from(0.50603841145833333338e-5_f64) * t34522 - F::cast_from(0.10120768229166666668e-4_f64) * t34525 - F::cast_from(0.97834092881944444454e-4_f64) * t34528 + F::cast_from(0.11382560960801989336e-6_f64) * t34530 + F::cast_from(0.13752370924682227773e-7_f64) * t34533 - F::cast_from(0.26222067084418402782e-7_f64) * t34537 + F::cast_from(0.28137654660407340486e-7_f64) * t34539 + F::cast_from(0.11279831795178992596e-7_f64) * t34547;
    (t38168, t38181)
}
