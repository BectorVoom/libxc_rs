//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 420/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk420<F: Float>(t641: F, t919: F, t21: F, t811: F, t1518: F, t1521: F, t1524: F, t1525: F, t1532: F, t1535: F, t1540: F, t1545: F, t1548: F, t287: F, t288: F, t808: F, t812: F, t815: F) -> (F, F) {
    let t2166 = t641 * t919;
    let t2185 = t811 * t21;
    let t2188 = F::new(0.35593054341882149702e-1) * t1518 * t288 + F::new(0.76848640056336459583e-2) * t1521 * t808 - F::new(0.8089330532245943114e-3) * t1525 * t812 + F::new(0.14382829686333286857e-2) * t1525 * t815 - F::new(0.8089330532245943114e-4) * t1524 * t287 * t1532 + F::new(0.14382829686333286857e-3) * t1535 * t815 + F::new(0.80893305322459431139e-5) * t1540 * t287 * t1545 - F::new(0.14382829686333286857e-4) * t1548 * t2185;
    (t2166, t2188)
}
