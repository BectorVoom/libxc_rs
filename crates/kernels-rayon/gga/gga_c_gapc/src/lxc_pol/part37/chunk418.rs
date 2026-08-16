//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 418/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk418(t641: f64, t919: f64, t21: f64, t811: f64, t1518: f64, t1521: f64, t1524: f64, t1525: f64, t1532: f64, t1535: f64, t1540: f64, t1545: f64, t1548: f64, t287: f64, t288: f64, t808: f64, t812: f64, t815: f64) -> (f64, f64) {
    let t2166 = t641 * t919;
    let t2185 = t811 * t21;
    let t2188 = 0.35593054341882149702e-1_f64 * t1518 * t288 + 0.76848640056336459583e-2_f64 * t1521 * t808 - 0.8089330532245943114e-3_f64 * t1525 * t812 + 0.14382829686333286857e-2_f64 * t1525 * t815 - 0.8089330532245943114e-4_f64 * t1524 * t287 * t1532 + 0.14382829686333286857e-3_f64 * t1535 * t815 + 0.80893305322459431139e-5_f64 * t1540 * t287 * t1545 - 0.14382829686333286857e-4_f64 * t1548 * t2185;
    (t2166, t2188)
}
