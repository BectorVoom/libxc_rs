//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1197/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1197<F: Float>(t1208: F, t2440: F, t24378: F, t25070: F, t28567: F, t108596: F, t28552: F, t112223: F, t19038: F, t287: F, t6793: F, t108922: F, t14760: F, t2405: F, t2413: F, t25077: F, t2693: F, t2719: F, t28558: F, t28561: F, t6035: F, t6774: F, t704: F, t98598: F, t98600: F, t98612: F, t992: F) -> (F,) {
    let t112346 = t2440 * t1208;
    let t112358 = 0.22226000364197530866e-1 * t25070 * t24378 * t28567;
    let t112365 = 0.22226000364197530866e-1 * t28552 * t108596;
    let t112366 = t19038 * t112223;
    let t112367 = t6793 * t287;
    let t112371 = -0.53706137268299704369e-1 * t28558 * t108922 + 0.33339000546296296297e-1 * t25077 * t6035 * t28561 * t2413 + 0.44452000728395061729e-1 * t25077 * t6035 * t112346 * t2405 - 0.33339000546296296298e-1 * t25070 * t6035 * t704 * t992 * t2719 - t112358 - 0.24163653553615319118e1 * t14760 * t6774 + 0.11113000182098765433e-1 * t98598 - 0.74086667880658436217e-2 * t98600 + 0.74086667880658436219e-2 * t98612 + t112365 + 0.10947790369858991998e1 * t112366 * t112367 * t2693;
    (t112371,)
}
