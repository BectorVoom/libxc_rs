//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1623/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1623<F: Float>(t1248: F, t12621: F, t371: F, t481: F, t482: F, t9291: F, t12627: F, t1284: F, t3624: F, t12629: F, t12910: F, t12911: F, t12916: F) -> (F, F, F, F, F) {
    let t44599 = t12621 * t1248;
    let t44607 = F::cast_from(0.14820648238345094262e-3_f64) * t481 * t371 * t9291 * t482;
    let t44609 = t12627 * t1284 * t3624;
    let t44610 = t12629 * t1248;
    let t44616 = t12910 * t12916 * t12911;
    (t44599, t44607, t44609, t44610, t44616)
}
