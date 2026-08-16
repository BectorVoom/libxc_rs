//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1049/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1049<F: Float>(t12571: F, t33676: F, t191: F, t192: F, t29497: F, t2169: F, t5456: F, t127601: F, t127603: F, t127606: F, t127669: F, t127671: F, t127673: F, t127677: F, t127679: F, t127681: F, t127684: F, t127686: F, t127688: F, t127690: F, t127698: F, t24972: F, t28951: F, t29422: F, t7423: F, t8508: F) -> (F, F, F, F) {
    let t129096 = t12571 * t33676;
    let t129164 = t29497 * t191 * t192;
    let t129282 = t2169 * t5456;
    let t130275 = t8508 + t127669 + t127671 + t127673 + t127601 + F::cast_from(0.135e2_f64) * t7423 * t28951 + t127603 + t127677 + t127679 + t127681 + t127684 + t127606 + t127686 + t127688 + t127690 + F::cast_from(54.0_f64) * t24972 * t29422 + t127698;
    (t129096, t129164, t129282, t130275)
}
