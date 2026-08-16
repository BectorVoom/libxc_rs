//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2416/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2416(t14459: f64, t17954: f64, t959: f64, t17955: f64, t4483: f64, t17304: f64, t17948: f64, t21094: f64, t952: f64, t21238: f64, t2904: f64, t17938: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t68934 = 0.51947577317044391277e2_f64 * t959 * t17954 * t14459;
    let t68936 = 0.51947577317044391276e2_f64 * t4483 * t17955;
    let t68938 = 0.10526802520742363173e2_f64 * t4483 * t17304;
    let t68940 = 0.31168546390226634765e3_f64 * t4483 * t17948;
    let t68943 = 0.14035736694323150897e2_f64 * t959 * t21094 * t952;
    let t68947 = 0.11696447245269292414e1_f64 * t959 * t2904 * t21238 * t952;
    let t68949 = 0.35089341735807877242e1_f64 * t4483 * t17938;
    (t68934, t68936, t68938, t68940, t68943, t68947, t68949)
}
