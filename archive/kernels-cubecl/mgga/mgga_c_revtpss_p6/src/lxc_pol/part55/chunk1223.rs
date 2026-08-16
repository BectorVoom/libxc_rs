//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1223/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1223<F: Float>(t103586: F, t125961: F, t125984: F, t126017: F, t126030: F, t127593: F, t127596: F, t1544: F, t1940: F, t2403: F, t25445: F, t26425: F, t26585: F, t26590: F, t27375: F, t28291: F, t28460: F, t32487: F, t32491: F, t32498: F, t32505: F, t34090: F, t34097: F, t4433: F, t4541: F, t7091: F, t7432: F, t8657: F, t95976: F) -> F {
    let t128060 = F::cast_from(2.0_f64) * t103586 * t1940 * t32505 + F::cast_from(2.0_f64) * t125961 * t1940 * t26590 - F::cast_from(3.0_f64) * t125984 * t2403 * t7432 + F::cast_from(2.0_f64) * t126017 * t1940 * t26590 - F::cast_from(3.0_f64) * t126030 * t2403 * t7432 + F::cast_from(2.0_f64) * t127593 * t1940 * t26590 - F::cast_from(3.0_f64) * t127596 * t2403 * t7432 + F::cast_from(3.0_f64) * t1544 * t2403 * t32487 + F::cast_from(2.0_f64) * t1940 * t34097 * t95976 - F::cast_from(3.0_f64) * t2403 * t26585 * t34090 - F::cast_from(3.0_f64) * t2403 * t27375 * t32491 - F::cast_from(3.0_f64) * t2403 * t28460 * t32498 + F::cast_from(6.0_f64) * t25445 * t26425 * t27375 - F::cast_from(6.0_f64) * t28291 * t4433 * t7091 + F::cast_from(6.0_f64) * t4433 * t4541 * t8657;
    t128060
}
