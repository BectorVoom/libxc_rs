//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1487/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1487<F: Float>(t108710: F, t109150: F, t109153: F, t109242: F, t1310: F, t1312: F, t1453: F, t18245: F, t2178: F, t2179: F, t2181: F, t22506: F, t2322: F, t27123: F, t28219: F, t30138: F, t30143: F, t31293: F, t31314: F, t31555: F, t31556: F, t4248: F, t4254: F, t5787: F, t651: F, t7732: F, t7889: F, t8254: F, t8274: F, t8278: F, t8280: F, t8362: F, t8367: F) -> F {
    let t118276 = F::cast_from(4.0_f64) * t1312 * t8362 * t5787 - F::cast_from(4.0_f64) * t4248 * t31314 + F::cast_from(2.0_f64) * t18245 * t8280 - F::cast_from(4.0_f64) * t7732 * t31314 - F::cast_from(2.0_f64) * t18245 * t8274 - F::cast_from(4.0_f64) * t109150 * t2179 - F::cast_from(4.0_f64) * t109153 * t2179 - F::cast_from(4.0_f64) * t30138 * t8254 + F::cast_from(4.0_f64) * t7889 * t31293 + F::cast_from(2.0_f64) * t1312 * t2178 * t22506 - F::cast_from(2.0_f64) * t2322 * t31556 - F::cast_from(2.0_f64) * t4254 * t31556 - F::cast_from(2.0_f64) * t651 * t1310 * t31555 + F::cast_from(4.0_f64) * t27123 * t8367 + F::cast_from(4.0_f64) * t28219 * t8367 + F::cast_from(4.0_f64) * t30138 * t8280 + F::cast_from(2.0_f64) * t108710 * t2181 + F::cast_from(2.0_f64) * t109242 * t2181 + F::cast_from(2.0_f64) * t30143 * t8278 + F::cast_from(2.0_f64) * t1312 * t31555 * t1453;
    t118276
}
