//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 1260/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk1260(t11188: f64, t1587: f64, t3634: f64, t11192: f64, t2906: f64, t1504: f64, t1803: f64, t996: f64, t35552: f64, t35555: f64, t35557: f64, t35559: f64, t35562: f64, t35564: f64, t35566: f64, t35570: f64, t35572: f64, t35575: f64) -> f64 {
    let t35578 = t11188 * t3634 * t1587;
    let t35580 = t2906 * t11192;
    let t35584 = t996 * t1803 * t3634 * t1504;
    let t35586 = 0.18103800586153667463e-6_f64 * t35552 + 0.18103800586153667463e-6_f64 * t35555 - 0.59742541934307102628e-4_f64 * t35557 - 0.37553317015878090875e-5_f64 * t35559 - 0.1545050757224698596e-4_f64 * t35562 - 0.80138718955667428014e-5_f64 * t35564 - 0.4049114220917933205e-4_f64 * t35566 + 0.4049114220917933205e-4_f64 * t35570 + 0.19570718734436677157e-3_f64 * t35572 - 0.12147342662753799615e-3_f64 * t35575 - 0.60736713313768998074e-4_f64 * t35578 - 0.12147342662753799615e-3_f64 * t35580 + 0.12147342662753799615e-3_f64 * t35584;
    t35586
}
