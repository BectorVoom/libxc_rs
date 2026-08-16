//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 617/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk617(t1173: f64, t713: f64, t6008: f64, t193: f64, t3977: f64, t6187: f64, t6940: f64, t766: f64, t2568: f64, t1449: f64, t3972: f64, t10002: f64, t6930: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27974 = t1173 * t713;
    let t27975 = t6008 * t27974;
    let t27976 = t193 * t27975;
    let t27981 = t3977 * t6187;
    let t27983 = t6940 * t766;
    let t27984 = t2568 * t27983;
    let t27986 = t1449 * t3972;
    let t27987 = t2568 * t27986;
    let t27989 = t10002 * t6930;
    (t27974, t27976, t27981, t27983, t27984, t27986, t27987, t27989)
}
