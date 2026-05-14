//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1062/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1062<F: Float>(t6861: F, t8085: F, t102219: F, t102225: F, t102249: F, t109450: F, t109453: F, t109455: F, t109458: F, t109460: F, t109488: F, t109505: F, t2097: F, t22953: F, t25924: F, t27837: F, t30262: F, t543: F, t6895: F, t7295: F, t7301: F, t96257: F) -> (F, F) {
    let t115107 = t8085 * t6861;
    let t115126 = -0.43368140941025997312e-1 * t109450 + 0.57824187921367996415e-1 * t102219 + 0.77108554593144223218e-1 * t109453 + 0.38554277296572111609e-1 * t109455 - 0.21684070470512998656e-1 * t109458 + 0.38554277296572111609e-1 * t109460 - 0.10281140612419229763e-1 * t102225 + 0.13010442282307799193e1 * t7295 * t7301 * t115107 * t543 + 0.13010442282307799193e1 * t27837 * t30262 + 0.4336814094102599731e0 * t7295 * t7301 * t2097 * t22953 * t543 - t96257 - 0.21951497276451705329e-1 * t102249 - 0.16463622957338778996e-1 * t109488 - 0.78062653693846795158e1 * t7295 * t25924 * t8085 * t6895 + 0.32927245914677557992e-1 * t109505;
    (t115107, t115126)
}
