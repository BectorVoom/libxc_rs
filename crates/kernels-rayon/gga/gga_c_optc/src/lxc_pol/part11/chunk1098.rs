//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1098/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1098(t4933: f64, t7274: f64, t930: f64, t4929: f64, t2619: f64, t2751: f64, t4997: f64, t2758: f64, t5002: f64, t5007: f64, t7878: f64, t940: f64) -> (f64, f64, f64, f64, f64) {
    let t42487 = t930 * t7274 * t4933;
    let t42490 = t930 * t7274 * t4929;
    let t42743 = t2751 * t2619 * t4997;
    let t42785 = t2758 * t2619 * t5002;
    let t42878 = t940 * t7878 * t5007;
    (t42487, t42490, t42743, t42785, t42878)
}
