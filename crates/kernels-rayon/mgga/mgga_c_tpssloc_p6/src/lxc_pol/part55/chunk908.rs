//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 908/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk908(t5308: f64, t8945: f64, t24995: f64, t1874: f64, t19456: f64, t4028: f64, t6525: f64, t5161: f64, t6996: f64, t1983: f64, t1914: f64, t193: f64, t200: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24996 = t8945 * t5308;
    let t24998 = 6.0_f64 * t24995 * t24996;
    let t25005 = 2.0_f64 * t19456 * t1874;
    let t25007 = 2.0_f64 * t4028 * t6525;
    let t25010 = t6996 * t5161;
    let t25011 = t1983 * t25010;
    let t25013 = t193 * t200 * t1914;
    (t24996, t24998, t25005, t25007, t25010, t25011, t25013)
}
