//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1059/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1059<F: Float>(t34077: F, t34127: F, t34132: F, t34156: F, t36934: F, t36937: F, t36938: F, t36939: F, t36940: F, t36942: F, t36951: F, t39026: F, t39029: F, t39031: F, t39035: F, t39039: F, t39041: F, t39043: F) -> (F,) {
    let t41452 = t34077 - t36934 - 0.21437009059034868486e-3 * t39026 - 0.21437009059034868486e-3 * t39029 + 0.18868855373762491241e-1 * t39031 + 0.85748036236139473944e-3 * t39035 - 0.31448092289604152068e-2 * t39039 + 11.0 / 192.0 * t39041 + 11.0 / 576.0 * t39043 + t36937 - t36938 - t36939 + t36940 + t36942 + 0.57165357490759649296e-3 * t34127 + t36951 - 0.75475421495049964964e-2 * t34132 - 0.37737710747524982482e-2 * t34156;
    (t41452,)
}
