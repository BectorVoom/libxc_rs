//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1073/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1073<F: Float>(t1659: F, t8331: F, t33698: F, t33699: F, t638: F, t315: F, t323: F, t9367: F, t38092: F, t7963: F, t7965: F, t4210: F, t7942: F) -> (F, F, F, F, F) {
    let t38251 = F::new(0.13170898365871023197e1) * t8331 * t1659;
    let t38256 = F::new(0.10408353825846239354e2) * t33698 * t638 * t33699;
    let t38259 = F::new(0.13170898365871023197e1) * t315 * t9367 * t323;
    let t38280 = F::new(0.17347256376410398924e1) * t7963 * t38092 * t7965;
    let t38283 = F::new(0.17347256376410398924e1) * t7942 * t38092 * t4210;
    (t38251, t38256, t38259, t38280, t38283)
}
