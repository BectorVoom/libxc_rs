//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1181/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1181<F: Float>(t3736: F, t42859: F, t13038: F, t1794: F, t8931: F, t3153: F, t1042: F, t124560: F, t124621: F, t124717: F, t124802: F, t124931: F, t1252: F, t131556: F, t131576: F, t131578: F, t131584: F, t17459: F, t1795: F, t21119: F, t29279: F, t31993: F, t32015: F, t33425: F, t33509: F, t371: F, t3719: F, t372: F, t482: F, t5236: F, t5428: F, t5465: F, t5497: F, t8938: F) -> (F, F, F, F) {
    let t131591 = t42859 * t3736;
    let t131592 = t131591 * t13038;
    let t131594 = t8931 * t1794;
    let t131595 = t131594 * t3153;
    let t131599 = F::new(0.19833242244827205771e-2) * t131556 * t1252 + F::new(0.3718732920905101082e-3) * t33509 * t371 * t372 * t482 * t5497 - F::new(0.11156198762715303246e-2) * t124560 * t1042 * t1795 * t21119 + F::new(0.11156198762715303246e-2) * t124802 * t1042 * t1795 * t17459 + F::new(0.34694512752820797848e1) * t124931 * t29279 - F::new(0.37645955677973955999e-3) * t131576 + F::new(0.56468933516960933998e-3) * t33425 * t32015 * t131578 * t5428 + F::new(0.82638509353446690713e-4) * t131584 - F::new(0.11156198762715303246e-2) * t124621 * t31993 * t3719 * t5236 + F::new(0.18822977838986977999e-3) * t124717 + F::new(0.34271842599061411569e1) * t8938 * t131592 * t131595 * t5465;
    (t131591, t131594, t131595, t131599)
}
