//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1301/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1301<F: Float>(t100800: F, t5838: F, t2030: F, t22591: F, t26738: F, t1013: F, t5555: F, t538: F, t93178: F, t6608: F, t94765: F, t1008: F, t104737: F, t104797: F, t104845: F, t104901: F, t12381: F, t1643: F, t1651: F, t1736: F, t2036: F, t23715: F, t23723: F, t23869: F, t26721: F, t3392: F, t3405: F, t40227: F, t5570: F, t5790: F, t925: F, t93169: F, t94429: F, t94434: F, t94530: F, t94602: F, t94666: F, t94686: F, t94689: F) -> (F, F, F, F) {
    let t105058 = t5838 * t100800;
    let t105061 = t22591 * t26738 * t2030;
    let t105064 = t5555 * t1013;
    let t105066 = t93178 * t105064 * t538;
    let t105080 = t94765 * t6608;
    let t105099 = 0.37043333940329218109e-2 * t105058 + 0.45306850413028723348e0 * t23869 * t105061 + 0.21895580739717983994e1 * t40227 * t105066 - 0.33339000546296296297e-1 * t23715 * t5570 * t26721 * t1651 - 0.44452000728395061729e-1 * t23715 * t5570 * t1736 * t1008 * t1643 - 0.1611184118048991131e0 * t94429 * t104901 + 0.28251445438725559511e-1 * t23723 * t105080 + 0.13335600218518518519e0 * t94434 * t93169 * t925 * t12381 + 0.1611184118048991131e0 * t94530 * t104737 + 0.33339000546296296298e-1 * t94666 - 0.17780800291358024692e0 * t94686 - 0.33339000546296296298e-1 * t94689 + 0.54738951849294959988e0 * t2036 * t5790 * t3405 + 0.40736230704976508653e-1 * t3392 * t104797 - 0.11300578175490223804e0 * t94602 * t104845;
    (t105061, t105064, t105066, t105099)
}
