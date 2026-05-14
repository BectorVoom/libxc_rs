//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1370/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1370<F: Float>(t6241: F, t83310: F, t1701: F, t19201: F, t6027: F, t2035: F, t4125: F, t6979: F, t123787: F, t28603: F, t112366: F, t112367: F, t127199: F, t127234: F, t127239: F, t127466: F, t19107: F, t231: F, t2446: F, t25049: F, t25070: F, t25077: F, t27506: F, t28591: F, t28595: F, t28616: F, t28652: F, t28660: F, t31508: F, t31526: F, t4094: F, t4110: F, t5260: F, t6035: F, t6045: F, t6057: F, t6233: F, t820: F, t82817: F, t83349: F) -> (F, F, F) {
    let t127471 = t83310 * t6241;
    let t127492 = t1701 * t6027 * t19201;
    let t127504 = t2035 * t6979 * t4125;
    let t127507 = t28603 * t123787;
    let t127515 = -0.33339000546296296297e-1 * t127471 * t6057 + 0.20003400327777777778e0 * t25049 * t6045 * t231 * t5260 * t820 + 0.28195722065857344794e1 * t28652 * t127199 - 0.28195722065857344794e1 * t28660 * t127466 + 0.66678001092592592596e-1 * t25070 * t6035 * t2446 * t127234 - 0.66678001092592592596e-1 * t25077 * t6035 * t2446 * t127239 + 0.45306850413028723348e0 * t4094 * t127492 + 0.45306850413028723348e0 * t28591 * t31508 - 0.90613700826057446696e0 * t83349 * t28595 + 0.45306850413028723348e0 * t31526 * t6233 + 0.90613700826057446696e0 * t82817 * t28595 - 0.21895580739717983995e1 * t19107 * t127504 + 0.26853068634149852184e-1 * t127507 - 0.10668480174814814815e1 * t25049 * t27506 * t28616 + 0.10947790369858991998e1 * t112366 * t112367 * t4110;
    (t127492, t127504, t127515)
}
