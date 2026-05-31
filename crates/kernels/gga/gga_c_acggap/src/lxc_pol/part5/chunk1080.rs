//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1080/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1080<F: Float>(t11516: F, t11520: F, t11522: F, t11525: F, t11526: F, t2633: F, t2644: F, t2828: F, t2835: F, t4028: F, t4030: F, t4032: F, t4034: F) -> F {
    let t19353 = F::cast_from(0.39503346997227602814e-1_f64) * t4028 + t11516 - F::cast_from(0.2077903092681775651e3_f64) * t2633 + F::cast_from(0.14649157844805236043e-2_f64) * t4030 - t4032 + F::cast_from(12.0_f64) * t4034 - t11520 + F::cast_from(6.0_f64) * t2644 + t11522 + F::cast_from(2.0_f64) * t2828 + t11525 - t11526 + F::cast_from(0.70178683471615754484e1_f64) * t2835;
    t19353
}
