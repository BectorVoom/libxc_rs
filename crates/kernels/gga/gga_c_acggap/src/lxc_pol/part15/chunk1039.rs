//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1039/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1039<F: Float>(t301: F, t642: F, t1679: F, t1717: F, t9460: F, t2138: F, t2147: F, t322: F, t9985: F, t157: F, t1620: F, t1937: F, t2146: F, t2152: F, t2217: F, t2222: F, t32315: F, t32324: F, t36794: F, t36808: F, t36809: F, t36811: F, t40740: F, t524: F, t6558: F, t7912: F, t7931: F, t8306: F, t9003: F, t9367: F, t9391: F, t9418: F, t9977: F) -> (F, F, F) {
    let t41001 = t301 * t642;
    let t41006 = t1679 * t9460 * t1717;
    let t41027 = t2138 * t2147 * t9985 * t322;
    let t41042 = 0.8673628188205199462e0 * t2146 * t2152 * t9367 * t524 * t157 + 0.17347256376410398924e1 * t32315 + 0.17347256376410398924e1 * t9003 * t9418 + t32324 + 0.10408353825846239354e2 * t36794 - 0.26020884564615598386e1 * t7912 * t9977 - 0.34694512752820797848e1 * t41027 - 0.8673628188205199462e0 * t7931 * t8306 * t40740 - 0.65854491829355115987e0 * t2222 * t6558 - t36808 + 0.8673628188205199462e0 * t2146 * t2147 * t2217 * t1937 - 0.52041769129231196772e1 * t36809 - 0.17347256376410398924e1 * t36811 + 0.26341796731742046394e1 * t9391 * t1620;
    (t41001, t41006, t41042)
}
