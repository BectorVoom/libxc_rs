//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 941/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk941<F: Float>(t9556: F, t9558: F, t9561: F, t9565: F, t9568: F, t9570: F, t9572: F, t9579: F, t9581: F, t9584: F, t9587: F, t9589: F, t9592: F) -> F {
    let t10872 = -F::new(0.98393192997685185193e-6) * t9556 - F::new(0.33816362383187442026e-4) * t9558 + F::new(0.14492726735651760868e-5) * t9561 + F::new(0.16882049790461501058e-6) * t9565 + F::new(0.33764099580923002116e-6) * t9568 - F::new(0.61320337121513228211e-3) * t9570 + F::new(0.9275345110817126956e-4) * t9572 - F::new(0.16882049790461501058e-6) * t9579 - F::new(0.19678638599537037038e-4) * t9581 + F::new(0.76020809606452561851e-3) * t9584 + F::new(0.28985453471303521736e-5) * t9587 + F::new(0.67632724766374884052e-4) * t9589 - F::new(0.28985453471303521736e-5) * t9592;
    t10872
}
