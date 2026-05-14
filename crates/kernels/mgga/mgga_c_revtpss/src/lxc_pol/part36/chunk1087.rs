//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1087/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1087<F: Float>(t33: F, t265: F, t502: F, t30865: F, t30922: F, t1300: F, t1832: F, t198: F, t27041: F, t29317: F, t29930: F, t336: F, t5023: F, t6748: F, t6752: F, t7673: F, t1469: F, t2159: F, t29977: F, t57: F, t5825: F, t8227: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t30923 = t30865 + t30922;
    let t30936 = piecewise3(t503, t1300 * t198 * t30923 * t336 - 2.0 * t1832 * t29317 * t5023 + 2.0 * t27041 * t5023 * t6752 - t5023 * t6748 * t7673, t29930);
    let t30943 = piecewise3(t400, t29977, t30936 * t57 / 2.0 - t8227 * t1469 - t2159 * t5825 / 2.0);
    (t30923, t30936, t30943)
}
