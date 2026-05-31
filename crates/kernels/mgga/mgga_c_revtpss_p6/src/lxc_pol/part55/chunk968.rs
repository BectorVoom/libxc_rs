//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 968/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk968<F: Float>(t1310: F, t1453: F, t2014: F, t2322: F, t25082: F, t28652: F, t28704: F, t28707: F, t28709: F, t28711: F, t28718: F, t28727: F, t4248: F, t4254: F, t4293: F, t4297: F, t508: F, t649: F, t651: F, t7359: F, t7378: F, t7969: F, t7984: F, t8065: F, t8075: F) -> F {
    let t28729 = -t1310 * t7969 + t1453 * t8075 - t2014 * t28707 - t2014 * t28709 - t2014 * t28727 - F::cast_from(2.0_f64) * t2322 * t7984 - F::cast_from(3.0_f64) * t25082 * t28718 - t28652 * t508 - F::cast_from(2.0_f64) * t28704 * t651 - F::cast_from(2.0_f64) * t28711 * t651 - F::cast_from(2.0_f64) * t4248 * t7378 - F::cast_from(2.0_f64) * t4254 * t7984 - F::cast_from(2.0_f64) * t4293 * t7359 - F::cast_from(2.0_f64) * t4297 * t7359 - t649 * t8065;
    t28729
}
