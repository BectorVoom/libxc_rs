//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 825/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk825(t2007: f64, t4466: f64, t15776: f64, t528: f64, t120: f64, t15656: f64, t72: f64, t4687: f64, t8959: f64, t422: f64, t4441: f64, t8966: f64) -> (f64, f64, f64, f64, f64) {
    let t16842 = t2007 * t4466;
    let t16845 = t528 * t15776;
    let t16848 = t15656 * t120;
    let t16849 = t72 * t16848;
    let t16853 = 0.8854768453090786061e-3_f64 * t8959 * t4687;
    let t16854 = t422 * t4441;
    let t16855 = t16854 * t8966;
    (t16842, t16845, t16849, t16853, t16855)
}
