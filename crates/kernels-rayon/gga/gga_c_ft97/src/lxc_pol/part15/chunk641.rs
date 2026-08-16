//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 641/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk641(t4702: f64, t8907: f64, t4441: f64, t8690: f64, t4687: f64, t8959: f64, t422: f64, t1008: f64, t132: f64, t4698: f64, t549: f64, t375: f64, t4715: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16798 = t8907 * t4702;
    let t16832 = t8690 * t4441;
    let t16853 = 0.8854768453090786061e-3_f64 * t8959 * t4687;
    let t16854 = t422 * t4441;
    let t16891 = t1008 * t132;
    let t16907 = t549 * t4698;
    let t16925 = t89 * t375 * t4715;
    (t16798, t16832, t16853, t16854, t16891, t16907, t16925)
}
