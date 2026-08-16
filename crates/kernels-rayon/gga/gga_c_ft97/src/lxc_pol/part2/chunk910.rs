//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 910/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk910(t255: f64, t9952: f64, t258: f64, t9570: f64, t13863: f64, t2526: f64, t992: f64, t2607: f64, t2606: f64, t14048: f64, t14052: f64, t14055: f64, t14060: f64, t14064: f64, t14068: f64, t14072: f64, t14077: f64, t1901: f64, t446: f64, t9813: f64, t9822: f64, t9824: f64, t9826: f64, t9828: f64) -> f64 {
    let t14080 = t9952 * t255;
    let t14081 = t258 * t9570;
    let t14082 = t14081 * t13863;
    let t14083 = t14080 * t14082;
    let t14086 = t992 * t2526;
    let t14087 = t2607 * t14086;
    let t14088 = t2606 * t14087;
    let t14091 = 2.0_f64 / 9.0_f64 * t9813 - 8.0_f64 / 27.0_f64 * t9822 - 8.0_f64 / 27.0_f64 * t9824 + t9826 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t9828 - t446 * t14048 / 9.0_f64 - t14052 + 4.0_f64 / 3.0_f64 * t446 * t14055 + 2.0_f64 / 3.0_f64 * t446 * t14060 + 2.0_f64 / 9.0_f64 * t446 * t14064 + t1901 * t14068 / 9.0_f64 + 2.0_f64 / 27.0_f64 * t1901 * t14072 - 2.0_f64 / 27.0_f64 * t1901 * t14077 - 10.0_f64 / 81.0_f64 * t1901 * t14083 + t1901 * t14088 / 9.0_f64;
    t14091
}
