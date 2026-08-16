//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1238/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1238(t32962: f64, t28231: f64, t24885: f64, t787: f64, t1457: f64, t2634: f64, t28242: f64, t28245: f64, t11109: f64, t22315: f64, t2617: f64, t7810: f64, t8802: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32963 = 0.29792074959875355558e-1_f64 * t32962;
    let t32968 = 0.31952438294933958064e0_f64 * t28231;
    let t32969 = t787 * t24885;
    let t32970 = t1457 * t2634;
    let t32972 = 0.50050685932590597338e1_f64 * t32969 * t32970;
    let t32973 = 0.25561950635947166452e0_f64 * t28242;
    let t32974 = 0.25561950635947166452e0_f64 * t28245;
    let t32978 = t22315 * t11109;
    let t32979 = 0.38342925953920749676e0_f64 * t32978;
    let t32983 = t7810 * t8802 * t2617;
    (t32963, t32968, t32970, t32972, t32973, t32974, t32979, t32983)
}
