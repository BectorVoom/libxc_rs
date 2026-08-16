//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 625/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk625(t5828: f64, t977: f64, t3003: f64, t4384: f64, t5718: f64, t5721: f64, t5724: f64, t340: f64, t343: f64, t974: f64, t1597: f64, t2969: f64, t2986: f64, t4507: f64, t4529: f64, t5818: f64, t5821: f64, t5825: f64, t973: f64) -> (f64, f64, f64, f64, f64) {
    let t5829 = t977 * t5828;
    let t5836 = -t3003 - 2.0_f64 / 9.0_f64 * t4384 + t5718 / 18.0_f64 - t5721 / 3.0_f64 + t5724 / 6.0_f64;
    let t5837 = t340 * t5836;
    let t5838 = t5837 * t343;
    let t5839 = t974 * t5838;
    let t5842 = t1597 * t1597;
    let t5843 = t340 * t5842;
    let t5844 = t5843 * t343;
    let t5845 = t974 * t5844;
    let t5848 = -t2969 + 0.18518518518518518518e-3_f64 * t4507 - 0.55555555555555555554e-3_f64 * t4529 + 0.37037037037037037036e-3_f64 * t973 * t5818 - 0.55555555555555555554e-3_f64 * t2986 * t5821 - 0.55555555555555555554e-3_f64 * t973 * t5825 + 0.27777777777777777777e-3_f64 * t973 * t5829 - 0.83333333333333333332e-3_f64 * t973 * t5839 - 0.83333333333333333332e-3_f64 * t973 * t5845;
    (t5836, t5838, t5842, t5844, t5848)
}
