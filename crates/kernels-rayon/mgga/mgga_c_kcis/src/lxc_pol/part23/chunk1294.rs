//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1294/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1294(t1650: f64, t28752: f64, t4468: f64, t6159: f64, t12844: f64, t27583: f64, t28806: f64, t18120: f64, t27567: f64, t7986: f64, t98260: f64, t98263: f64, t98268: f64, t99201: f64, t99205: f64, t99210: f64, t99213: f64, t99219: f64) -> (f64, f64) {
    let t99224 = t6159 * t28752 * t1650 * t4468;
    let t99229 = 0.7722800925925925926e-4_f64 * t27583 * t12844 * t28806;
    let t99231 = 0.77382407407407407407e-3_f64 * t98260 - 0.30952962962962962962e-2_f64 * t98263 - 0.61836467013888888889e-4_f64 * t27567 * t99201 - 0.30918233506944444444e-4_f64 * t27567 * t99205 + 0.41224311342592592592e-4_f64 * t27567 * t99210 - 0.46336805555555555556e-3_f64 * t27583 * t99213 * t18120 - 0.23168402777777777778e-3_f64 * t27583 * t99205 - 0.18534722222222222222e-2_f64 * t99219 * t7986 + 0.15459116753472222222e-4_f64 * t27567 * t99224 + t99229 - 0.51588271604938271604e-3_f64 * t98268;
    (t99224, t99231)
}
