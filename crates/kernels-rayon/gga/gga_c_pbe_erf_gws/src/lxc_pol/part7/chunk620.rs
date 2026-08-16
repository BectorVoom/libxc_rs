//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 620/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk620(t4821: f64, t1423: f64, t409: f64, t1333: f64, t461: f64, t1438: f64, t428: f64, t4688: f64, t4711: f64, t4714: f64, t4718: f64, t4811: f64, t4815: f64, t4818: f64, t4820: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4822 = 24.0_f64 * t4821;
    let t4823 = t409 * t1423;
    let t4824 = 12.0_f64 * t4823;
    let t4825 = t1333 * t461;
    let t4826 = 60.0_f64 * t4825;
    let t4827 = t1438 * t428;
    let t4828 = 96.0_f64 * t4827;
    let t4829 = t4811 - t4815 + t4688 + t4711 - t4714 - t4718 - t4818 + t4820 - t4822 + t4824 + t4826 - t4828;
    (t4822, t4823, t4824, t4825, t4826, t4827, t4828, t4829)
}
