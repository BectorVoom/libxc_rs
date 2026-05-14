//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1041/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1041<F: Float>(t13426: F, t8457: F, t18227: F, t32311: F, t4248: F, t28030: F, t7003: F, t125362: F, t1937: F, t125365: F, t33602: F, t6993: F, t127369: F, t127371: F, t127373: F, t127375: F, t127378: F, t127384: F, t127385: F, t32316: F, t33903: F, t4292: F, t5787: F, t651: F, t670: F, t7007: F, t8557: F, t8565: F) -> (F,) {
    let t127393 = t13426 * t8457;
    let t127395 = t18227 * t8457;
    let t127397 = t4248 * t32311;
    let t127399 = t28030 * t7003;
    let t127401 = t125362 * t1937;
    let t127403 = t125365 * t1937;
    let t127405 = t33602 * t6993;
    let t127409 = -2.0 * t33903 * t651 * t670 - 2.0 * t4292 * t651 * t8557 - 4.0 * t28030 * t7007 - 2.0 * t32316 * t4248 + t5787 * t8565 - t127369 - t127371 - t127373 - t127375 - t127378 - t127384 - t127385 - 4.0 * t127393 - 4.0 * t127395 - 4.0 * t127397 - 4.0 * t127399 - 4.0 * t127401 - 4.0 * t127403 - 4.0 * t127405;
    (t127409,)
}
