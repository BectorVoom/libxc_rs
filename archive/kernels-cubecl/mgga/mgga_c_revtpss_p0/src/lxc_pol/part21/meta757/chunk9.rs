//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2664/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2664<F: Float>(t13847: F, t13848: F, t3924: F, t9816: F, t13910: F, t808: F, t9736: F, t14026: F, t9744: F, t125: F, t13716: F, t13975: F, t1399: F, t3934: F, t3936: F, t4004: F, t4057: F, t47259: F, t47262: F, t47277: F, t47282: F, t47284: F, t47286: F, t49012: F, t49016: F, t49024: F, t49030: F, t49049: F, t5671: F, t5673: F, t5674: F, t9891: F) -> F {
    let t49053 = t9816 * t13847 * t13848 * t3924;
    let t49056 = t9736 * t808 * t13910;
    let t49057 = F::cast_from(0.30492001685571196935e-4_f64) * t49056;
    let t49058 = t9744 * t14026;
    let t49060 = -F::cast_from(0.30492001685571196935e-4_f64) * t49012 + F::cast_from(0.22869001264178397701e-3_f64) * t49016 - F::cast_from(0.51448821741683684367e-2_f64) * t5671 * t3936 * t13975 * t4004 - F::cast_from(0.30492001685571196935e-3_f64) * t49024 - F::cast_from(0.21437009059034868486e-3_f64) * t3934 * t5673 * t5674 * t9891 + F::cast_from(455.0_f64) / F::cast_from(648.0_f64) * t49030 - F::cast_from(0.54214778996945588148e-4_f64) * t47259 + F::cast_from(0.97586602194502058666e-3_f64) * t47262 - F::cast_from(0.76230004213927992337e-3_f64) * t47277 - F::cast_from(0.38115002106963996168e-4_f64) * t47282 + F::cast_from(0.25724410870841842183e-2_f64) * t3934 * t3936 * t125 * t13716 * t1399 + F::cast_from(0.25724410870841842183e-2_f64) * t3934 * t3936 * t13975 * t4057 - F::cast_from(0.12004725073059526352e-1_f64) * t47284 + F::cast_from(0.30011812682648815881e-2_f64) * t47286 + F::cast_from(0.30492001685571196935e-3_f64) * t49049 - F::cast_from(0.38115002106963996168e-4_f64) * t49053 + t49057 - F::cast_from(7.0_f64) / F::cast_from(8.0_f64) * t49058;
    t49060
}
