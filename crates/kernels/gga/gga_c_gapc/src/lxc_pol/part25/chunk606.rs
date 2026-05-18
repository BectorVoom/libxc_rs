//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 606/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk606<F: Float>(t3314: F, t3316: F, t3318: F, t3323: F, t3331: F, t3334: F, t3338: F, t3341: F, t3346: F, t3349: F, t3351: F, t3355: F, t3358: F, t3361: F, t3365: F, t3369: F, t3372: F, t3376: F, t3380: F, t3385: F, t3389: F, t3393: F) -> (F, F) {
    let t3594 = F::new(0.10120442708333333334e-4) * t3314 - F::new(0.5060221354166666667e-4) * t3316 - F::new(0.64871090864172852779e-2) * t3318 - F::new(0.50027140879067581468e-8) * t3323 - F::new(0.24619655944423022376e-7) * t3331 + F::new(0.21135226489492151266e-6) * t3334 + F::new(0.17376185052903442709e-3) * t3338 + F::new(0.17376185052903442709e-3) * t3341 - F::new(0.25745714186718600948e-5) * t3346 + F::new(0.2318836277704281739e-4) * t3349 - F::new(0.4637672555408563478e-4) * t3351;
    let t3607 = F::new(0.4637672555408563478e-4) * t3355 + F::new(0.38647271295071362317e-6) * t3358 - F::new(0.68714848362636882201e-6) * t3361 - F::new(0.84410248952307505288e-7) * t3365 - F::new(0.84410248952307505288e-7) * t3369 + F::new(0.61900849231692170545e-6) * t3372 + F::new(0.28136749650769168429e-7) * t3376 - F::new(0.27801896084645508334e-2) * t3380 + F::new(0.12163329537032409896e-2) * t3385 - F::new(0.10120442708333333334e-4) * t3389 - F::new(0.10120442708333333334e-4) * t3393;
    (t3594, t3607)
}
