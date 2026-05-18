//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 795/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk795<F: Float>(t169: F, t174: F, t176: F, t2641: F, t6281: F, t6284: F, t44: F, t6280: F, t230: F, t6276: F, t234: F, t441: F, t233: F, t1658: F, t1876: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t170 = t169 <= zeta_threshold;
    let t175 = t174 <= zeta_threshold;
    let t6288 = piecewise3::<f64>(t175, F::new(0.0), F::new(4.0) / F::new(9.0) * t2641 * t6281 + F::new(4.0) / F::new(3.0) * t176 * t6284);
    let t6290 = (t6280 + t6288) * t44;
    let t6291 = t6290 * t230;
    let t6293 = piecewise3::<f64>(t170, F::new(0.0), t6276);
    let t6294 = t234 * t6293;
    let t6295 = t6294 * t441;
    let t6296 = t233 * t6295;
    let t6297 = t6296 / F::new(16.0);
    let t6298 = t1658 * t1876;
    (t6290, t6291, t6294, t6295, t6297, t6298)
}
