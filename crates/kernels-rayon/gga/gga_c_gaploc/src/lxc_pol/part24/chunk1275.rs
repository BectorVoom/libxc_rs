//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1275/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1275(t32962: f64, t28231: f64, t24885: f64, t787: f64, t1457: f64, t2634: f64, t28242: f64, t28245: f64, t11109: f64, t22315: f64, t11001: f64, t1445: f64, t2061: f64, t2070: f64, t2201: f64, t28249: f64, t28259: f64, t32387: f64, t32951: f64, t32953: f64, t32955: f64, t32958: f64, t32960: f64) -> (f64, f64) {
    let t32963 = 0.29792074959875355558e-1_f64 * t32962;
    let t32968 = 0.31952438294933958064e0_f64 * t28231;
    let t32969 = t787 * t24885;
    let t32970 = t1457 * t2634;
    let t32972 = 0.50050685932590597338e1_f64 * t32969 * t32970;
    let t32973 = 0.25561950635947166452e0_f64 * t28242;
    let t32974 = 0.25561950635947166452e0_f64 * t28245;
    let t32978 = t22315 * t11109;
    let t32979 = 0.38342925953920749676e0_f64 * t32978;
    let t32980 = -t32951 + t32953 + t32955 - t32958 - t32960 + t32963 + 0.71500979903700853338e0_f64 * t2070 * t11001 + 0.35750489951850426669e0_f64 * t2061 * t11001 - t32968 - t32972 - t32973 + t32974 - t28249 - t28259 - 0.46011511144704899612e1_f64 * t2201 * t1445 * t32387 - t32979;
    (t32970, t32980)
}
