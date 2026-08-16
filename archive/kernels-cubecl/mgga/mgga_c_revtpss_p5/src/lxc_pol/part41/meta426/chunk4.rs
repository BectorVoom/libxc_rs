//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1490/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1490<F: Float>(t114: F, t118353: F, t118405: F, t13426: F, t18227: F, t1843: F, t21658: F, t2178: F, t2181: F, t2322: F, t30138: F, t31248: F, t31292: F, t31293: F, t31299: F, t31320: F, t31324: F, t31518: F, t31570: F, t31579: F, t4248: F, t4254: F, t508: F, t5517: F, t651: F, t75439: F, t7732: F, t7889: F, t8274: F, t8353: F, t8362: F, t8367: F) -> (F, F) {
    let t115 = F::cast_from(1.0_f64) < t114;
    let t118407 = piecewise3::<F>(t115, F::cast_from(0.0_f64), t118353 + t118405);
    let t118413 = -F::cast_from(2.0_f64) * t118407 * t508 * t651 - F::cast_from(4.0_f64) * t1843 * t31292 * t651 - F::cast_from(2.0_f64) * t21658 * t2178 * t651 - F::cast_from(4.0_f64) * t5517 * t651 * t8362 - F::cast_from(4.0_f64) * t13426 * t8353 + F::cast_from(4.0_f64) * t13426 * t8367 - F::cast_from(4.0_f64) * t18227 * t8353 + F::cast_from(4.0_f64) * t18227 * t8367 + F::cast_from(2.0_f64) * t2181 * t75439 - F::cast_from(2.0_f64) * t2322 * t31518 + F::cast_from(4.0_f64) * t2322 * t31570 - F::cast_from(4.0_f64) * t2322 * t31579 - F::cast_from(4.0_f64) * t30138 * t8274 + F::cast_from(4.0_f64) * t31248 * t7889 + F::cast_from(4.0_f64) * t31293 * t4248 - F::cast_from(4.0_f64) * t31299 * t4248 - F::cast_from(4.0_f64) * t31320 * t7732 + F::cast_from(4.0_f64) * t31324 * t7889 - F::cast_from(2.0_f64) * t31518 * t4254 - F::cast_from(4.0_f64) * t31579 * t4254;
    (t118407, t118413)
}
