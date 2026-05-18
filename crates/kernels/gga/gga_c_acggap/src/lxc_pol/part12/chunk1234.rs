//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1234/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1234<F: Float>(t2131: F, t2147: F, t309: F, t9413: F, t1659: F, t8331: F, t33698: F, t33699: F, t638: F, t315: F, t323: F, t9367: F) -> (F, F, F, F) {
    let t38241 = F::new(0.34694512752820797848e1) * t2131 * t2147 * t9413 * t309;
    let t38251 = F::new(0.13170898365871023197e1) * t8331 * t1659;
    let t38256 = F::new(0.10408353825846239354e2) * t33698 * t638 * t33699;
    let t38259 = F::new(0.13170898365871023197e1) * t315 * t9367 * t323;
    (t38241, t38251, t38256, t38259)
}
