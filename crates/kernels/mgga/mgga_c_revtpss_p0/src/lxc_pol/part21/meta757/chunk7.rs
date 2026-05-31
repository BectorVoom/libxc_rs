//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2662/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2662<F: Float>(t13785: F, t48862: F, t48999: F, t13817: F, t13999: F, t13981: F, t9962: F, t13951: F, t2713: F, t3964: F, t1353: F, t14019: F, t1872: F, t3889: F, t3944: F, t47223: F, t47227: F, t47229: F, t47231: F, t47235: F, t47239: F, t47245: F, t48971: F, t48975: F, t48982: F, t48984: F, t5689: F, t800: F, t9628: F) -> F {
    let t49001 = t48862 * t48999 * t13785;
    let t49003 = t13999 * t13817;
    let t49005 = t9962 * t13981;
    let t49008 = t3964 * t2713 * t13951;
    let t49010 = -F::cast_from(0.60023625365297631762e-2_f64) * t47223 - F::cast_from(0.12705000702321332056e-4_f64) * t47227 - F::cast_from(0.17006693853500995666e-1_f64) * t47229 - F::cast_from(0.24009450146119052704e-1_f64) * t48971 - F::cast_from(0.76230004213927992337e-4_f64) * t48975 - F::cast_from(0.12004725073059526352e-1_f64) * t47231 - F::cast_from(0.30492001685571196935e-3_f64) * t47235 + F::cast_from(0.76230004213927992336e-4_f64) * t47239 - F::cast_from(0.8131200449485652516e-3_f64) * t48982 - F::cast_from(0.12004725073059526352e-1_f64) * t48984 + F::cast_from(0.60023625365297631762e-1_f64) * t47245 + F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t3944 * t800 * t14019 * t1353 + F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t3944 * t800 * t5689 * t3889 + t3944 * t800 * t1872 * t9628 / F::cast_from(16.0_f64) + F::cast_from(0.85748036236139473944e-3_f64) * t49001 - F::cast_from(0.18007087609589289528e-1_f64) * t49003 - F::cast_from(0.12004725073059526352e-1_f64) * t49005 - F::cast_from(0.54214778996945588151e-4_f64) * t49008;
    t49010
}
