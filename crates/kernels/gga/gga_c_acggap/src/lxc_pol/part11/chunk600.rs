//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 600/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk600<F: Float>(t172: F, t3706: F, t1017: F, t513: F, t398: F, t1456: F, t997: F, t2606: F, t2610: F, t2613: F, t2616: F, t2622: F, t2625: F, t2629: F, t2634: F, t2642: F, t2826: F, t3994: F, t3995: F, t4029: F, t4031: F, t4032: F, t4036: F) -> (F, F, F, F, F) {
    let t5011 = t172 * t3706;
    let t5012 = t513 * t1017;
    let t5014 = t398 * t5011 * t5012;
    let t5017 = t997 * t1456;
    let t5019 = -t2606 + t2610 + t3994 + t2613 + t2616 + t3995 - t2622 - t2625 + t4029 + t2629 - t2634 + t4031 - t4032 - t2642 + t4036 + t2826;
    (t5011, t5012, t5014, t5017, t5019)
}
