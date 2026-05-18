//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 756/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk756<F: Float>(t263: F, t7484: F, t684: F, t2354: F, t5994: F, t7150: F, t1439: F, t1774: F, t7426: F, t1425: F, t666: F, t461: F, t6144: F) -> (F, F, F, F, F, F, F, F) {
    let t33535 = t7484 * t263;
    let t33536 = t33535 * t684;
    let t33537 = t2354 * t33536;
    let t33540 = t5994 * t7150;
    let t33543 = t1774 * t1439;
    let t33545 = t7426 * t33543 / F::new(18.0);
    let t33546 = t1425 * t684;
    let t33547 = t666 * t33546;
    let t33552 = t461 * t6144;
    (t33535, t33537, t33540, t33543, t33545, t33546, t33547, t33552)
}
