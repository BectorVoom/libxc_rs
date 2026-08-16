//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2648/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2648<F: Float>(t157: F, t73989: F, t74009: F, t182: F, t20675: F, t3701: F, t39305: F, t1388: F, t20077: F, t20681: F, t3918: F, t39249: F, t39256: F, t39261: F, t39266: F, t39304: F, t5160: F, t5187: F, t53783: F, t53788: F, t53797: F, t55224: F, t73958: F, t73959: F, t73960: F, t73961: F, t73962: F, t73968: F, t73969: F) -> (F, F, F, F) {
    let t74011 = (t73989 + t74009) * t157;
    let t74013 = F::cast_from(0.19751673498613801407e-1_f64) * t74011 * t182;
    let t74014 = t20675 * t3701;
    let t74017 = F::cast_from(0.10389515463408878255e3_f64) * t39305;
    let t74020 = -t1388 * t5160 * t74014 - F::cast_from(9.0_f64) * t20077 * t3918 * t5187 + F::cast_from(18.0_f64) * t20681 * t55224 - t39249 - t39256 - t39261 - t39266 - t39304 + t53783 + t53788 + t53797 - t73958 - t73959 - t73960 - t73961 - t73962 - t73968 - t73969 + t74013 + t74017;
    (t74011, t74013, t74017, t74020)
}
