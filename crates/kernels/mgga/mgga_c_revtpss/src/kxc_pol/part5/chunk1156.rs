//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1156/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1156<F: Float>(t14586: F, t14786: F, t14791: F, t1559: F, t4433: F, t14785: F, t2652: F, t6030: F, t10858: F, t6024: F, t10816: F, t10824: F, t10826: F, t18456: F, t18459: F, t18462: F, t18466: F, t18471: F, t18475: F, t2745: F, t4362: F) -> F {
    let t18477 = t14586 * t14786;
    let t18478 = t14791 * t18477;
    let t18481 = t1559 * t4433;
    let t18482 = t14785 * t18481;
    let t18485 = t2652 * t6030;
    let t18487 = t10858 * t6024;
    let t18489 = F::new(0.12862205435420921092e-2) * t4362 * t18456 + F::new(0.10003937560882938627e-2) * t18459 - F::new(0.42874018118069736972e-3) * t2745 * t18462 - F::new(0.21437009059034868486e-3) * t2745 * t18466 - F::new(0.42874018118069736972e-2) * t2745 * t18471 - F::new(0.56688979511669985553e-2) * t10816 - F::new(0.20007875121765877254e-1) * t18475 - F::new(0.34299214494455789578e-2) * t4362 * t18478 - F::new(0.85748036236139473945e-2) * t2745 * t18482 + F::new(0.40015750243531754507e-2) * t18485 - t10824 + t10826 - F::new(0.20007875121765877254e-2) * t18487;
    t18489
}
