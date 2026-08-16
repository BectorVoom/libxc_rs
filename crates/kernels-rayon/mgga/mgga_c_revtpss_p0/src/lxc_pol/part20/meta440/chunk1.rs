//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1672/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1672(t13038: f64, t42859: f64, t460: f64, t44376: f64, t487: f64, t13045: f64, t43351: f64, t1204: f64, t1234: f64, t1248: f64, t12646: f64, t12702: f64, t12737: f64, t12747: f64, t12756: f64, t1285: f64, t1287: f64, t12966: f64, t13107: f64, t13108: f64, t13112: f64, t13133: f64, t13142: f64, t13143: f64, t3153: f64, t3584: f64, t3588: f64, t3670: f64, t3727: f64, t3751: f64, t3759: f64, t44421: f64, t45584: f64, t5480: f64) -> (f64, f64) {
    let t45607 = t42859 * t13038;
    let t45608 = t460 * t45607;
    let t45609 = t487 * t44376;
    let t45610 = t43351 * t13045;
    let t45617 = 0.26341796731742046395e1_f64 * t1285 * t13107 * t1248 * t1287 - 0.15805078039045227836e2_f64 * t13142 * t45584 * t13143 + 0.79025390195226139183e1_f64 * t44421 * t3751 + 0.26341796731742046395e1_f64 * t1204 * t13108 + 0.39512695097613069592e1_f64 * t1285 * t3727 * t3588 * t1287 + 0.15805078039045227836e2_f64 * t12966 * t12737 + 0.15805078039045227836e2_f64 * t3670 * t3759 * t12646 + 0.79025390195226139184e1_f64 * t12756 * t12747 * t3153 * t5480 + 0.15805078039045227836e2_f64 * t12702 * t13112 - 0.23707617058567841754e2_f64 * t45608 * t45609 * t45610 - 0.39512695097613069592e1_f64 * t1234 * t13133 * t3584;
    (t45609, t45617)
}
