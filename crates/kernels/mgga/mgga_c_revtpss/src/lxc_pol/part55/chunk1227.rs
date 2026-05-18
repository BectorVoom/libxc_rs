//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1227/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1227<F: Float>(t102851: F, t102888: F, t110165: F, t127212: F, t127566: F, t127582: F, t127907: F, t1940: F, t2403: F, t26425: F, t26585: F, t27764: F, t27799: F, t27802: F, t28460: F, t32487: F, t32491: F, t32553: F, t32559: F, t32561: F, t34151: F, t34153: F, t7207: F, t7432: F, t7862: F) -> F {
    let t128150 = -F::new(3.0) / F::new(2.0) * t102888 * t32553 - t1940 * t127582 * t7207 / F::new(2.0) + t110165 * t32559 + F::new(3.0) / F::new(2.0) * t2403 * t32487 * t7862 - t1940 * t7432 * t127212 / F::new(2.0) + t102851 * t34151 - t1940 * t28460 * t32561 / F::new(2.0) + F::new(3.0) * t26425 * t27799 * t127907 - t1940 * t26585 * t34153 / F::new(2.0) + F::new(3.0) * t127566 * t27764 - t1940 * t32491 * t27802 / F::new(2.0);
    t128150
}
