//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 943/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk943<F: Float>(t9680: F, t9683: F, t9686: F, t9690: F, t9693: F, t9698: F, t9704: F, t9707: F, t9711: F, t9714: F, t9717: F, t9719: F, t9724: F) -> F {
    let t10915 = -F::new(0.73909120450717768468e-5) * t9680 + F::new(0.15176747947735985782e-6) * t9683 - F::new(0.2698425785107458272e-6) * t9686 - F::new(0.51491428373437201896e-5) * t9690 + F::new(0.4637672555408563478e-4) * t9693 - F::new(0.75091666377929252765e-6) * t9698 - F::new(0.66398272271344937795e-7) * t9704 + F::new(0.1180561280984512994e-6) * t9707 - F::new(0.18757833100512778952e-8) * t9711 + F::new(0.25294579912893309636e-8) * t9714 + F::new(0.10120442708333333334e-4) * t9717 + F::new(0.27801896084645508334e-2) * t9719 + F::new(0.16882049790461501058e-6) * t9724;
    t10915
}
