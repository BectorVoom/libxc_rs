//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1126/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1126<F: Float>(t24237: F, t27997: F, t2567: F, t6907: F, t2569: F, t1449: F, t53484: F, t14145: F, t96834: F, t108335: F, t1403: F, t193: F, t24191: F, t24231: F, t24398: F, t27953: F, t27956: F, t27974: F, t5996: F, t6002: F, t6009: F, t6745: F, t684: F, t98153: F, t98157: F, t98159: F, t98161: F) -> (F, F, F, F) {
    let t109643 = 2.0 / 3.0 * t24237 * t27997;
    let t109652 = t6907 * t2567;
    let t109653 = t109652 * t2569;
    let t109659 = t53484 * t1449;
    let t109661 = t96834 * t14145;
    let t109667 = t98153 / 27.0 + t98157 / 27.0 + t98159 / 27.0 + t98161 / 54.0 - t109643 + t6745 * t24398 / 6.0 - 2.0 / 3.0 * t5996 * t27953 - 2.0 / 3.0 * t1403 * t193 * t108335 * t6009 + 4.0 * t109653 + 2.0 / 9.0 * t6002 * t24231 * t27956 * t684 - 2.0 * t109659 - 12.0 * t109661 - 2.0 / 3.0 * t1403 * t193 * t24191 * t27974;
    (t109653, t109659, t109661, t109667)
}
