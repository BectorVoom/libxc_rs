//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1672/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1672<F: Float>(t13038: F, t42859: F, t460: F, t44376: F, t487: F, t13045: F, t43351: F, t1204: F, t1234: F, t1248: F, t12646: F, t12702: F, t12737: F, t12747: F, t12756: F, t1285: F, t1287: F, t12966: F, t13107: F, t13108: F, t13112: F, t13133: F, t13142: F, t13143: F, t3153: F, t3584: F, t3588: F, t3670: F, t3727: F, t3751: F, t3759: F, t44421: F, t45584: F, t5480: F) -> (F, F) {
    let t45607 = t42859 * t13038;
    let t45608 = t460 * t45607;
    let t45609 = t487 * t44376;
    let t45610 = t43351 * t13045;
    let t45617 = F::cast_from(0.26341796731742046395e1_f64) * t1285 * t13107 * t1248 * t1287 - F::cast_from(0.15805078039045227836e2_f64) * t13142 * t45584 * t13143 + F::cast_from(0.79025390195226139183e1_f64) * t44421 * t3751 + F::cast_from(0.26341796731742046395e1_f64) * t1204 * t13108 + F::cast_from(0.39512695097613069592e1_f64) * t1285 * t3727 * t3588 * t1287 + F::cast_from(0.15805078039045227836e2_f64) * t12966 * t12737 + F::cast_from(0.15805078039045227836e2_f64) * t3670 * t3759 * t12646 + F::cast_from(0.79025390195226139184e1_f64) * t12756 * t12747 * t3153 * t5480 + F::cast_from(0.15805078039045227836e2_f64) * t12702 * t13112 - F::cast_from(0.23707617058567841754e2_f64) * t45608 * t45609 * t45610 - F::cast_from(0.39512695097613069592e1_f64) * t1234 * t13133 * t3584;
    (t45609, t45617)
}
