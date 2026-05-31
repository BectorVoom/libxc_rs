//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2223/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2223<F: Float>(t5: F, t111468: F, t111493: F, t111521: F, t111548: F, t111577: F, t111623: F, t111652: F, t111680: F, t117: F, t105859: F, t105863: F, t105889: F, t105894: F, t105897: F, t108067: F, t108068: F, t108076: F, t1310: F, t13426: F, t18227: F, t18245: F, t21891: F, t27060: F, t29432: F, t29444: F, t30716: F, t34446: F, t4248: F, t4293: F, t508: F, t5787: F, t5887: F, t7586: F, t7591: F, t8158: F, t8237: F) -> (F, F) {
    let t7 = piecewise3::<F>(F::cast_from(0.0_f64) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t111684 = piecewise3::<F>(t8, F::cast_from(0.0_f64), t111468 + t111493 + t111521 + t111548 + t111577 + t111623 + t111652 + t111680);
    let t111685 = t111684 * t117;
    let t111690 = -t111685 * t508 - t1310 * t30716 - F::cast_from(4.0_f64) * t13426 * t8158 - F::cast_from(4.0_f64) * t18227 * t8158 - F::cast_from(2.0_f64) * t18245 * t7591 - F::cast_from(4.0_f64) * t21891 * t7586 - F::cast_from(4.0_f64) * t27060 * t5887 - F::cast_from(4.0_f64) * t29432 * t5887 - F::cast_from(4.0_f64) * t29444 * t4248 - F::cast_from(4.0_f64) * t34446 * t4293 + F::cast_from(2.0_f64) * t5787 * t8237 - t105859 - t105863 - t105889 + t105894 + t105897 + t108067 + t108068 - t108076;
    (t111685, t111690)
}
