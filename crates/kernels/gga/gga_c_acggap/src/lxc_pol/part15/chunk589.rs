//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 589/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk589<F: Float>(t2606: F, t2610: F, t2613: F, t2616: F, t2622: F, t2625: F, t2629: F, t2634: F, t2642: F, t2644: F, t2826: F, t5443: F, t5444: F, t5476: F, t5477: F, t5478: F) -> (F,) {
    let t6004 = -t2606 + t2610 + t5443 + t2613 + t2616 + t5444 - t2622 - t2625 + t5476 + t2629 - t2634 + t5477 + t5478 - t2642 + t2644 + t2826;
    (t6004,)
}
