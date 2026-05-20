//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3224/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3224<F: Float>(t12621: F, t12699: F, t12748: F, t12757: F, t1280: F, t13161: F, t17192: F, t17849: F, t17949: F, t3670: F, t3755: F, t3756: F, t45738: F, t5351: F, t5457: F, t5458: F, t5474: F, t56543: F, t57275: F, t57325: F, t57465: F, t58785: F, t59650: F, t59657: F, t59671: F, t59674: F, t59681: F, t59686: F) -> F {
    let t59689 = -F::cast_from(0.19756347548806534796e1_f64) * t45738 * t59650 * t58785 + F::cast_from(0.19756347548806534796e1_f64) * t17949 * t59650 * t57325 - F::cast_from(0.39512695097613069591e1_f64) * t59657 * t3756 + F::cast_from(0.13170898365871023197e1_f64) * t3670 * t1280 * t56543 + F::cast_from(0.19756347548806534796e1_f64) * t12699 * t5474 - F::cast_from(0.39512695097613069591e1_f64) * t57465 * t13161 - F::cast_from(0.19756347548806534796e1_f64) * t3755 * t57275 * t5458 + F::cast_from(0.11853808529283920877e2_f64) * t59671 * t17849 + F::cast_from(0.19756347548806534796e1_f64) * t59674 * t12757 - F::cast_from(0.65854491829355115987e0_f64) * t3755 * t5351 * t5457 * t12621 + F::cast_from(0.19756347548806534796e1_f64) * t59681 * t12757 - F::cast_from(0.19756347548806534796e1_f64) * t17192 * t12748 - F::cast_from(0.39512695097613069591e1_f64) * t59686 * t3756;
    t59689
}
