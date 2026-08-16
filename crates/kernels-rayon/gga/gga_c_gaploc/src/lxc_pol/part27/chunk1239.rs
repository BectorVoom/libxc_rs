//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1239/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1239(t28099: f64, t24390: f64, t955: f64, t7366: f64, t8775: f64, t11057: f64, t1991: f64, t28141: f64, t1445: f64, t32210: f64, t833: f64, t28156: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t32911 = 0.15976219147466979032e-1_f64 * t28099;
    let t32923 = 0.47667319935800568892e0_f64 * t955 * t24390;
    let t32925 = 0.23833659967900284446e0_f64 * t8775 * t7366;
    let t32926 = t1991 * t11057;
    let t32927 = 0.1022478025437886658e1_f64 * t32926;
    let t32928 = 0.31952438294933958064e-1_f64 * t28141;
    let t32931 = 0.11502877786176224903e2_f64 * t833 * t1445 * t32210;
    let t32935 = 0.31952438294933958064e-1_f64 * t28156;
    (t32911, t32923, t32925, t32927, t32928, t32931, t32935)
}
