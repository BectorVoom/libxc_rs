//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1028/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1028(t13069: f64, t7416: f64, t13154: f64, t24799: f64, t24661: f64, t13153: f64, t3251: f64, t4752: f64, t13023: f64, t2103: f64, t4673: f64, t1445: f64, t43213: f64, t833: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43611 = t7416 * t13069;
    let t43617 = 0.42900587942220512003e1_f64 * t24799 * t13154;
    let t43619 = 0.42900587942220512003e1_f64 * t24661 * t13154;
    let t43627 = 0.28600391961480341335e1_f64 * t13153 * t4752 * t3251;
    let t43630 = 0.47667319935800568892e0_f64 * t2103 * t4673 * t13023;
    let t43636 = 0.11502877786176224903e2_f64 * t833 * t1445 * t43213;
    (t43611, t43617, t43619, t43627, t43630, t43636)
}
