//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1053/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1053<F: Float>(t13426: F, t8457: F, t18227: F, t32311: F, t4248: F, t28030: F, t7003: F, t125362: F, t1937: F, t125365: F, t33602: F, t6993: F, t2042: F, t28246: F, t1916: F, t32369: F) -> (F, F, F, F, F, F, F, F, F) {
    let t127393 = t13426 * t8457;
    let t127395 = t18227 * t8457;
    let t127397 = t4248 * t32311;
    let t127399 = t28030 * t7003;
    let t127401 = t125362 * t1937;
    let t127403 = t125365 * t1937;
    let t127405 = t33602 * t6993;
    let t127439 = t28246 * t2042;
    let t127442 = 12.0 * t1916 * t32369;
    (t127393, t127395, t127397, t127399, t127401, t127403, t127405, t127439, t127442)
}
