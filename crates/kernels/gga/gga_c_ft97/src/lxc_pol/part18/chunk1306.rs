//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1306/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1306<F: Float>(t5829: F, t6608: F, t92433: F, t2071: F, t22591: F, t26743: F, t104639: F, t105159: F, t12512: F, t12550: F, t22767: F, t23869: F, t26604: F, t26608: F, t26650: F, t40227: F, t48613: F, t5579: F, t5812: F, t5813: F, t6605: F, t6609: F, t72: F, t92429: F, t94429: F, t94852: F, t94854: F, t94856: F, t94873: F, t94950: F) -> (F, F) {
    let t105224 = 0.17780800291358024692e0 * t5829 * t92433 * t6608;
    let t105244 = t22591 * t26743 * t2071;
    let t105255 = 0.97794401602469135802e0 * t5829 * t92429 * t6608 - t105224 + 0.20003400327777777778e0 * t94950 * t6609 - 0.53342400874074074074e0 * t5829 * t22767 * t26650 + 0.10001700163888888889e0 * t5829 * t5579 * t72 * t12550 - 0.10001700163888888889e0 * t48613 * t5812 * t6605 - 0.20003400327777777778e0 * t26604 * t26608 - 0.10001700163888888889e0 * t5813 * t5579 * t72 * t12512 + 0.45306850413028723348e0 * t23869 * t105244 + 0.21895580739717983994e1 * t40227 * t105159 - 0.1611184118048991131e0 * t94429 * t104639 + 0.18834296959150373008e-1 * t94852 + 0.26853068634149852184e-1 * t94854 - 0.26853068634149852184e-1 * t94856 - 0.22226000364197530866e-1 * t94873;
    (t105244, t105255)
}
