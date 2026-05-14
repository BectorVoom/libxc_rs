//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 858/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk858<F: Float>(t7990: F, t9076: F, t1620: F, t7973: F, t2331: F, t323: F, t851: F, t2137: F, t32123: F, t1619: F, t322: F, t620: F, t1614: F, t7927: F, t2138: F, t2147: F, t463: F, t8418: F) -> (F, F, F, F, F, F) {
    let t33686 = 0.34694512752820797848e1 * t7990 * t9076;
    let t33691 = 0.26341796731742046394e1 * t7973 * t1620;
    let t33695 = t851 * t2331 * t323;
    let t33698 = t2137 * t32123;
    let t33699 = t1619 * t322;
    let t33702 = 0.10408353825846239354e2 * t33698 * t620 * t33699;
    let t33715 = t7927 * t1614;
    let t33726 = 0.34694512752820797848e1 * t2138 * t2147 * t8418 * t463;
    (t33686, t33691, t33695, t33702, t33715, t33726)
}
