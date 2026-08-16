//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 760/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk760(t15742: f64, t3088: f64, t419: f64, t11273: f64, t15746: f64, t1725: f64, t4484: f64, t173: f64, t4483: f64, t1527: f64, t15752: f64, t11280: f64, t15756: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15846 = t3088 * t15742;
    let t15847 = t419 * t15846;
    let t15849 = t11273 * t15746;
    let t15850 = t419 * t15849;
    let t15852 = t1725 * t4484;
    let t15854 = t173 * t4483;
    let t15855 = t419 * t15854;
    let t15857 = t1527 * t15752;
    let t15858 = t419 * t15857;
    let t15860 = t11280 * t15756;
    (t15847, t15850, t15852, t15855, t15858, t15860)
}
