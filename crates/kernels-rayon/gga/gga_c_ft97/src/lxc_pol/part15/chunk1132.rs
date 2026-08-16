//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1132/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1132(t1113: f64, t21309: f64, t1110: f64, t5025: f64, t51: f64, t6: f64, t52324: f64, t88796: f64, t21392: f64, t66422: f64, t5049: f64, t1127: f64, t13414: f64, t14722: f64, t17854: f64, t17987: f64, t17993: f64, t17997: f64, t18133: f64, t2035: f64, t21157: f64, t21222: f64, t21243: f64, t21306: f64, t21374: f64, t2387: f64, t2710: f64, t27711: f64, t27733: f64, t3759: f64, t3766: f64, t3767: f64, t3789: f64, t3790: f64, t4978: f64, t5005: f64, t5266: f64, t66424: f64, t66555: f64, t678: f64, t680: f64, t80149: f64, t80167: f64, t807: f64, t88462: f64, t88504: f64, t9524: f64, t9609: f64, t9681: f64) -> (f64, f64, f64) {
    let t88813 = t21309 * t1113;
    let t88819 = t5025 * t6 * t51 * t1110;
    let t88822 = t52324 * t88796;
    let t88825 = t66422 * t21392;
    let t88858 = t5049 * t6 * t51 * t1110;
    let t88875 = -0.20265659080606036994e-4_f64 * t17854 * t18133 - 0.2232470490858028032e-1_f64 * t3759 * t9609 * t88813 + 0.2845142710964406234e0_f64 * t66555 * t88819 - 0.32032606786708831383e-6_f64 * t88822 * t13414 + 0.57000134242798356259e-7_f64 * t88825 * t66424 - 0.1422571355482203117e0_f64 * t80167 * t21306 - 0.25803162535905570824e-4_f64 * t3759 * t807 * t88813 - 0.139529405678626752e0_f64 * t3759 * t680 * t4978 * t5005 - 8.0_f64 * t3766 * t3767 * t21222 - 48.0_f64 * t3766 * t9681 * t21157 * t1113 - 36.0_f64 * t3789 * t17997 * t5049 + 8.0_f64 * t3789 * t3790 * t21222 - 24.0_f64 * t27733 * t21243 - 0.84321219226603029515e-3_f64 * t17987 * t2035 * t5266 * t5049 - 0.14225713554822031171e0_f64 * t17993 * t88858 + 0.46509801892875584e-1_f64 * t2387 * t680 * t21374 * t1127 - 0.69716604262587839785e-3_f64 * t678 * t9524 * t88462 + 0.9009618584720619741e0_f64 * t27711 * t14722 * t80149 * t1113 + 0.38704743803858356237e-5_f64 * t678 * t2710 * t88504;
    (t88813, t88858, t88875)
}
