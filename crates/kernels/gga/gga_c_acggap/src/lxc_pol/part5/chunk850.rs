//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 850/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk850<F: Float>(t11934: F, t265: F, t272: F, t286: F, t11787: F, t13: F, t2803: F, t758: F, t775: F, t2955: F, t883: F, t685: F) -> (F, F, F, F) {
    let t11938 = F::new(0.5848223622634646207e0) * t286 * t265 * t11934 * t272;
    let t11944 = F::new(0.62071215503128080361e4) * t13 / t775 / t758 * t11787 * t2803;
    let t11945 = t883 * t2955;
    let t11947 = t685 * t685;
    (t11938, t11944, t11945, t11947)
}
