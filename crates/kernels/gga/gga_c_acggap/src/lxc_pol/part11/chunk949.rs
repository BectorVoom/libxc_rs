//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 949/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk949<F: Float>(t2060: F, t507: F, t7811: F, t31419: F, t4810: F, t721: F, t30659: F, t34610: F, t34612: F, t34614: F, t34617: F, t34618: F, t34621: F, t34623: F, t34627: F, t34630: F, t34633: F, t34636: F, t34638: F, t34640: F, t34644: F) -> (F,) {
    let t34647 = t2060 * t507 * t7811;
    let t34650 = t31419 * t4810 * t721;
    let t34653 = -t34610 - t34612 + 0.21437009059034868486e-2 * t34614 - t34617 - 0.11321313224257494744e-1 * t34618 + t34621 - t34623 - t34627 + 0.64311027177104605458e-2 * t34630 - t34633 - 0.47172138434406228102e-3 * t34636 + 0.15724046144802076034e-3 * t34638 + 0.28303283060643736862e-1 * t34640 - 0.47172138434406228102e-2 * t34644 + 0.7640625e-2 * t34647 + 0.114609375e-1 * t34650 + 0.25724410870841842183e-2 * t30659;
    (t34653,)
}
