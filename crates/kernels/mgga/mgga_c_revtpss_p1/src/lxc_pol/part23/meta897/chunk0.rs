//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2856/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2856<F: Float>(t50084: F, t61239: F, t50092: F, t50094: F, t23221: F, t2398: F, t61247: F, t61282: F, t61289: F, t50852: F, t50856: F, t61294: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t77002 = F::new(12.0) * t50084;
    let t77003 = F::cast_from(0.17544670867903938621e1_f64) * t61239;
    let t77004 = F::cast_from(0.48796115851357829289e-1_f64) * t50092;
    let t77005 = F::cast_from(0.14447919941302971323e1_f64) * t50094;
    let t77007 = F::new(4.0) * t2398 * t23221;
    let t77008 = F::cast_from(0.32530743900905219526e-1_f64) * t61247;
    let t77009 = F::cast_from(0.73245789224026180216e-3_f64) * t61282;
    let t77010 = F::new(24.0) * t61289;
    let t77011 = F::cast_from(0.15584273195113317383e3_f64) * t50852;
    let t77012 = F::cast_from(0.17090684152272775384e-2_f64) * t50856;
    let t77013 = F::cast_from(0.17544670867903938621e1_f64) * t61294;
    (t77002, t77003, t77004, t77005, t77007, t77008, t77009, t77010, t77011, t77012, t77013)
}
