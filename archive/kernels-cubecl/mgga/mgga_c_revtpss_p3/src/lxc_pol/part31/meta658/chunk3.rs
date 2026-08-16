//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2225/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2225<F: Float>(t13269: F, t1470: F, t4173: F, t4181: F, t4187: F, t21698: F, t603: F, t101326: F, t1928: F, t28105: F, t28109: F, t28112: F, t28116: F, t28119: F, t28138: F, t29554: F, t6974: F, t6978: F, t7706: F, t7716: F) -> F {
    let t108807 = t13269 * t1470;
    let t108810 = t4173 * t4181;
    let t108813 = t4173 * t4187;
    let t108816 = t603 * t21698;
    let t108829 = F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t101326 * t7706 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t28138 * t28105 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t28138 * t28109 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t108807 * t1928 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t108810 * t1928 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t108813 * t1928 + t108816 * t1928 / F::cast_from(3.0_f64) + t29554 * t6974 / F::cast_from(3.0_f64) + t29554 * t6978 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t28112 * t7716 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t28116 * t7716 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t28119 * t7716;
    t108829
}
