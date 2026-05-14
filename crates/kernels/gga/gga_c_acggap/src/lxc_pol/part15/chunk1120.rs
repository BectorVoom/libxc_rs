//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1120/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1120<F: Float>(t1734: F, t694: F, t8034: F, t9826: F, t10039: F, t104: F, t2407: F, t3952: F, t105: F, t1954: F, t2170: F, t2249: F, t36715: F, t36729: F, t36744: F, t38519: F, t38573: F, t41001: F, t41006: F, t41042: F, t41065: F, t41086: F, t41111: F, t41145: F, t41169: F, t41194: F, t41225: F, t41246: F, t41267: F, t41295: F, t42189: F, t42205: F, t42225: F, t42258: F, t42284: F, t469: F, t567: F, t6596: F, t8382: F, t9096: F, t9098: F, t9121: F, t9469: F) -> (F,) {
    let t42293 = t694 * t8034 * t1734;
    let t42298 = t694 * t9826;
    let t42300 = t104 * t10039;
    let t42307 = t2407 * t3952;
    let t42311 = 6.0 * t567 * t41001 * t9469 - t36715 + 2.0 * t41006 + 2.0 * t567 * t2249 * t6596 + 3.0 * t567 * t2170 * t38573 + t567 * t105 * (t41042 + t41065 + t41086 + t41111 + t41145 + t41169 + t41194 + t41225 + t41246 + t41267 + t41295 + t42189 + t42205 + t42225 + t42258 + t42284) * t469 + 3.0 * t42293 - 6.0 * t9096 * t36729 * t38519 - 6.0 * t42298 + 3.0 * t567 * t42300 * t1954 + 6.0 * t567 * t9121 * t8382 + 4.0 * t9096 * t42307 * t9098 - t36744;
    (t42311,)
}
