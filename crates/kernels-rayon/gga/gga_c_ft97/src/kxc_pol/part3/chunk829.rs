//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 829/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk829(t16907: f64, t554: f64, t2057: f64, t4702: f64, t12374: f64, t16827: f64, t16830: f64, t16889: f64, t16891: f64, t16894: f64, t16897: f64, t16902: f64, t2001: f64, t3380: f64, t399: f64, t4674: f64, t4677: f64, t4700: f64, t4704: f64, t538: f64, t555: f64) -> f64 {
    let t16908 = t16907 * t554;
    let t16911 = t2057 * t4702;
    let t16917 = -t16827 - 2.0_f64 * t4674 * t555 + 2.0_f64 * t16830 + 2.0_f64 * t16889 + 4.0_f64 * t16891 * t3380 - 4.0_f64 * t2001 * t16894 - 0.1208182677680765956e1_f64 * t16897 * t399 + 0.1208182677680765956e1_f64 * t4700 * t399 - 0.1208182677680765956e1_f64 * t16902 * t399 + 0.1208182677680765956e1_f64 * t4704 * t399 - 2.0_f64 * t2001 * t16908 + 4.0_f64 * t2001 * t16911 * t538 - 4.0_f64 * t12374 * t4677;
    t16917
}
