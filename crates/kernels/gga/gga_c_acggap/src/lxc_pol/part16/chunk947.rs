//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 947/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk947<F: Float>(t1619: F, t322: F, t33698: F, t620: F, t1614: F, t7927: F, t2138: F, t2147: F, t463: F, t8418: F, t315: F, t32123: F) -> (F, F, F, F) {
    let t33699 = t1619 * t322;
    let t33702 = F::cast_from(0.10408353825846239354e2_f64) * t33698 * t620 * t33699;
    let t33715 = t7927 * t1614;
    let t33726 = F::cast_from(0.34694512752820797848e1_f64) * t2138 * t2147 * t8418 * t463;
    let t33743 = t315 * t32123;
    (t33702, t33715, t33726, t33743)
}
