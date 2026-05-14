//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1053/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1053<F: Float>(t30106: F, t32348: F, t32349: F, t32350: F, t32352: F, t33894: F, t33908: F, t33916: F, t33922: F, t33927: F, t36841: F, t38820: F, t38830: F, t38834: F, t38840: F, t38846: F, t38848: F, t38852: F) -> (F,) {
    let t41371 = 0.25724410870841842183e-2 * t38820 - 0.57165357490759649296e-3 * t33894 - t36841 - t32348 - t32349 + t32350 + t32352 + t33908 - 0.12579236915841660828e-2 * t33916 + 0.37737710747524982482e-2 * t30106 + t33922 - t33927 - 0.42874018118069736972e-3 * t38830 - 0.21437009059034868486e-3 * t38834 - 0.12862205435420921092e-2 * t38840 + 0.94344276868812456204e-2 * t38846 - 0.13719685797782315831e-1 * t38848 - 0.31448092289604152068e-2 * t38852;
    (t41371,)
}
