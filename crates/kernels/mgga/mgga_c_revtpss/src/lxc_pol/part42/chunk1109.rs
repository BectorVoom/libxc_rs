//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1109/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1109<F: Float>(t1559: F, t4433: F, t14785: F, t2652: F, t6030: F, t10858: F, t6024: F, t10816: F, t10824: F, t10826: F, t18456: F, t18459: F, t18462: F, t18466: F, t18471: F, t18475: F, t18478: F, t2745: F, t4362: F) -> (F,) {
    let t18481 = t1559 * t4433;
    let t18482 = t14785 * t18481;
    let t18485 = t2652 * t6030;
    let t18487 = t10858 * t6024;
    let t18489 = 0.12862205435420921092e-2 * t4362 * t18456 + 0.10003937560882938627e-2 * t18459 - 0.42874018118069736972e-3 * t2745 * t18462 - 0.21437009059034868486e-3 * t2745 * t18466 - 0.42874018118069736972e-2 * t2745 * t18471 - 0.56688979511669985553e-2 * t10816 - 0.20007875121765877254e-1 * t18475 - 0.34299214494455789578e-2 * t4362 * t18478 - 0.85748036236139473945e-2 * t2745 * t18482 + 0.40015750243531754507e-2 * t18485 - t10824 + t10826 - 0.20007875121765877254e-2 * t18487;
    (t18489,)
}
