//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1252/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1252<F: Float>(t19742: F, t16613: F, t16615: F, t16592: F, t16599: F, t16607: F, t16612: F, t24543: F, t24544: F, t24546: F, t24598: F, t24601: F, t24602: F, t24605: F, t24607: F, t24608: F, t24609: F, t24610: F) -> (F, F, F, F) {
    let t24611 = 80.0 * t19742;
    let t24612 = 160.0 * t16613;
    let t24613 = 0.2077903092681775651e3 * t16615;
    let t24614 = -t24543 - t24544 + t24546 + t24598 + t24601 + t24602 + t24605 + t24607 - t16592 - t24608 - t24609 + t16599 - t24610 + t24611 + t16607 - t16612 - t24612 + t24613;
    (t24611, t24612, t24613, t24614)
}
