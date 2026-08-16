//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1225/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1225<F: Float>(t107923: F, t1113: F, t127227: F, t127284: F, t127596: F, t1940: F, t2403: F, t25759: F, t26425: F, t26585: F, t27773: F, t27777: F, t27810: F, t28472: F, t32505: F, t33888: F, t34080: F, t34090: F, t34145: F, t7200: F, t7432: F, t8657: F, t94245: F, t95511: F) -> F {
    let t128183 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t8657 * t27773 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t8657 * t27777 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t26425 * t25759 * t127596 + t28472 * t127284 - t1940 * t26585 * t33888 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t8657 * t27810 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t34080 * t7200 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t26425 * t94245 * t34090 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t95511 * t34145 + t28472 * t107923 * t32505 + t1940 * t34080 * t1113 / F::cast_from(2.0_f64) - t1940 * t7432 * t127227 / F::cast_from(2.0_f64);
    t128183
}
