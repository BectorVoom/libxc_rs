//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2246/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2246<F: Float>(t25759: F, t61182: F, t101029: F, t101032: F, t101035: F, t101040: F, t101051: F, t101055: F, t1711: F, t1940: F, t1963: F, t2403: F, t25206: F, t25436: F, t25445: F, t25763: F, t25778: F, t27158: F, t27773: F, t27800: F, t7087: F, t7207: F, t7783: F, t7862: F, t98719: F, t98722: F, t98784: F, t99555: F) -> F {
    let t101061 = t25759 * t61182;
    let t101064 = F::new(2.0) * t98719 * t27800 + F::new(3.0) * t2403 * t7783 * t25763 + F::new(6.0) * t27158 * t101029 + F::new(6.0) * t27158 * t101032 + F::new(3.0) * t27158 * t101035 - t1940 * t99555 * t7207 + t1940 * t25445 * t101040 + t1940 * t98722 * t25778 + t1940 * t25436 * t1711 / F::new(2.0) + F::new(3.0) / F::new(2.0) * t2403 * t25436 * t7862 + F::new(3.0) / F::new(2.0) * t2403 * t1963 * t101051 + t98784 - F::new(3.0) * t25206 * t101055 + F::new(3.0) * t2403 * t7087 * t27773 - F::new(3.0) * t25206 * t101061;
    t101064
}
