//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1080/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1080<F: Float>(t11516: F, t11520: F, t11522: F, t11525: F, t11526: F, t2633: F, t2644: F, t2828: F, t2835: F, t4028: F, t4030: F, t4032: F, t4034: F) -> F {
    let t19353 = F::new(0.39503346997227602814e-1) * t4028 + t11516 - F::new(0.2077903092681775651e3) * t2633 + F::new(0.14649157844805236043e-2) * t4030 - t4032 + F::new(12.0) * t4034 - t11520 + F::new(6.0) * t2644 + t11522 + F::new(2.0) * t2828 + t11525 - t11526 + F::new(0.70178683471615754484e1) * t2835;
    t19353
}
