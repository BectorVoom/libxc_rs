//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1287/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1287<F: Float>(t22572: F, t23705: F, t26696: F, t363: F, t554: F, t23715: F, t26722: F, t100530: F, t100588: F, t104639: F, t104647: F, t104658: F, t1647: F, t2059: F, t22568: F, t23711: F, t26692: F, t26695: F, t26701: F, t26706: F, t2992: F, t423: F, t5570: F, t920: F, t93169: F, t94508: F, t94524: F) -> (F, F) {
    let t104663 = t23705 * t22572 * t26696;
    let t104671 = t554 * t363;
    let t104682 = t23715 * t22572 * t26722;
    let t104684 = 0.1611184118048991131e0 * t94524 * t104639 - 0.17780800291358024692e0 * t23705 * t22568 * t26706 + t104647 - 0.10001700163888888889e0 * t94508 * t5570 * t423 * t920 * t2059 + 0.17780800291358024692e0 * t23715 * t22568 * t26701 - t104658 - 0.17780800291358024692e0 * t23705 * t22568 * t26696 + 0.22226000364197530864e-1 * t104663 - 0.66678001092592592594e-1 * t23705 * t5570 * t26695 * t1647 - 0.1611184118048991131e0 * t23711 * t100588 - 0.13335600218518518519e0 * t23705 * t93169 * t2992 * t104671 + 0.13335600218518518519e0 * t26692 * t100530 + 0.17780800291358024692e0 * t23715 * t22568 * t26722 - 0.22226000364197530864e-1 * t104682;
    (t104671, t104684)
}
