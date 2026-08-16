//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 659/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk659(t1306: f64, t135: f64, t273: f64, t2745: f64, t2748: f64, t2750: f64, t2753: f64, t2785: f64, t2789: f64, t2857: f64, t2859: f64, t2862: f64, t2864: f64, t2868: f64, t2872: f64, t2877: f64, t2993: f64, t2997: f64, t803: f64, t805: f64) -> f64 {
    let t3000 = t135 * t273 * t2993 * t805 - t1306 * t2997 * t803 - t2745 + t2748 + t2750 - t2753 + t2785 + t2789 + t2857 + t2859 - t2862 - t2864 + t2868 - t2872 - t2877;
    t3000
}
