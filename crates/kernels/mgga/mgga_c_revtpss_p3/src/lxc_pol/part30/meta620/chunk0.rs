//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2132/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2132<F: Float>(t1940: F, t2255: F, t7087: F, t27383: F, t61155: F, t27375: F, t92790: F, t14767: F, t27159: F, t4537: F, t605: F, t15071: F, t30: F) -> (F, F, F, F, F, F) {
    let t98684 = F::cast_from(2.0_f64) * t1940 * t7087 * t2255;
    let t98688 = t27383 * t61155;
    let t98694 = t92790 * t27375;
    let t98699 = t27159 * t14767;
    let t98702 = t605 * t4537;
    let t98705 = t30 * t15071;
    (t98684, t98688, t98694, t98699, t98702, t98705)
}
