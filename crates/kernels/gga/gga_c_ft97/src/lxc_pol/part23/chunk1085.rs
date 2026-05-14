//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1085/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1085<F: Float>(t1526: F, t21114: F, t9483: F, t21110: F, t21118: F, t342: F, t630: F, t13616: F, t16579: F, t17713: F, t17715: F, t17718: F, t17722: F, t17736: F, t17740: F, t17757: F, t18178: F, t2320: F, t2321: F, t42264: F, t42293: F, t42295: F, t69132: F, t69137: F, t69141: F, t69143: F) -> (F,) {
    let t81968 = t1526 * t9483 * t21114;
    let t81971 = t1526 * t9483 * t21110;
    let t81974 = t342 * t630 * t21118;
    let t81993 = -t1526 * t2320 * t17757 / 6.0 - t81968 / 36.0 + t81971 / 18.0 - t81974 / 12.0 + t42264 / 54.0 + t17713 + t18178 + t17718 + t17715 - t69132 + t69137 / 27.0 - t69141 + t69143 / 9.0 + t1526 * t13616 * t17736 / 3.0 + t1526 * t2320 * t17740 / 6.0 - t1526 * t2320 * t17722 / 12.0 - t1526 * t2320 * t2321 * t16579 / 12.0 - t42293 + t42295 / 18.0;
    (t81993,)
}
