//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1132/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1132<F: Float>(t1113: F, t21309: F, t1110: F, t5025: F, t51: F, t6: F, t52324: F, t88796: F, t21392: F, t66422: F, t5049: F, t1127: F, t13414: F, t14722: F, t17854: F, t17987: F, t17993: F, t17997: F, t18133: F, t2035: F, t21157: F, t21222: F, t21243: F, t21306: F, t21374: F, t2387: F, t2710: F, t27711: F, t27733: F, t3759: F, t3766: F, t3767: F, t3789: F, t3790: F, t4978: F, t5005: F, t5266: F, t66424: F, t66555: F, t678: F, t680: F, t80149: F, t80167: F, t807: F, t88462: F, t88504: F, t9524: F, t9609: F, t9681: F) -> (F, F, F) {
    let t88813 = t21309 * t1113;
    let t88819 = t5025 * t6 * t51 * t1110;
    let t88822 = t52324 * t88796;
    let t88825 = t66422 * t21392;
    let t88858 = t5049 * t6 * t51 * t1110;
    let t88875 = -F::cast_from(0.20265659080606036994e-4_f64) * t17854 * t18133 - F::cast_from(0.2232470490858028032e-1_f64) * t3759 * t9609 * t88813 + F::cast_from(0.2845142710964406234e0_f64) * t66555 * t88819 - F::cast_from(0.32032606786708831383e-6_f64) * t88822 * t13414 + F::cast_from(0.57000134242798356259e-7_f64) * t88825 * t66424 - F::cast_from(0.1422571355482203117e0_f64) * t80167 * t21306 - F::cast_from(0.25803162535905570824e-4_f64) * t3759 * t807 * t88813 - F::cast_from(0.139529405678626752e0_f64) * t3759 * t680 * t4978 * t5005 - F::cast_from(8.0_f64) * t3766 * t3767 * t21222 - F::cast_from(48.0_f64) * t3766 * t9681 * t21157 * t1113 - F::cast_from(36.0_f64) * t3789 * t17997 * t5049 + F::cast_from(8.0_f64) * t3789 * t3790 * t21222 - F::cast_from(24.0_f64) * t27733 * t21243 - F::cast_from(0.84321219226603029515e-3_f64) * t17987 * t2035 * t5266 * t5049 - F::cast_from(0.14225713554822031171e0_f64) * t17993 * t88858 + F::cast_from(0.46509801892875584e-1_f64) * t2387 * t680 * t21374 * t1127 - F::cast_from(0.69716604262587839785e-3_f64) * t678 * t9524 * t88462 + F::cast_from(0.9009618584720619741e0_f64) * t27711 * t14722 * t80149 * t1113 + F::cast_from(0.38704743803858356237e-5_f64) * t678 * t2710 * t88504;
    (t88813, t88858, t88875)
}
