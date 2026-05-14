//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1046/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1046<F: Float>(t4058: F, t6854: F, t14157: F, t321: F, t13760: F, t804: F, t14150: F, t353: F, t4053: F, t814: F, t859: F, t1193: F, t2100: F, t13791: F, t2387: F, t2227: F) -> (F, F, F, F, F, F, F, F) {
    let t50839 = t4058 * t6854;
    let t50846 = t321 * t14157;
    let t50868 = t804 * t13760;
    let t50870 = t321 * t14150;
    let t50876 = t859 * t353 * t4053 * t814;
    let t50881 = t859 * t353 * t1193 * t2100;
    let t50884 = t2387 * t13791;
    let t50891 = t859 * t2227;
    (t50839, t50846, t50868, t50870, t50876, t50881, t50884, t50891)
}
