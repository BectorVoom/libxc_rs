//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2202/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2202<F: Float>(t141: F, t16903: F, t12254: F, t16715: F, t16708: F, t16710: F, t16712: F, t12296: F, t12297: F, t12299: F, t12301: F, t12303: F, t16706: F, t16717: F, t16722: F, t16727: F, t16731: F, t16735: F, t16740: F, t16744: F, t16748: F) -> (F, F, F, F) {
    let t16904 = t141 * t16903;
    let t16907 = t12254 * t16715;
    let t16908 = t141 * t16907;
    let t16915 = F::new(4.0) / F::new(27.0) * t16708;
    let t16916 = F::new(4.0) / F::new(9.0) * t16710;
    let t16917 = F::new(2.0) / F::new(9.0) * t16712;
    let t16926 = -t12296 + F::new(8.0) / F::new(27.0) * t12297 + F::new(2.0) / F::new(27.0) * t12299 - F::new(2.0) / F::new(9.0) * t12301 - t12303 / F::new(9.0) + F::new(4.0) / F::new(27.0) * t16706 + t16915 - t16916 - t16917 + F::new(10.0) / F::new(27.0) * t16717 - F::new(4.0) / F::new(3.0) * t16722 - F::new(4.0) / F::new(9.0) * t16727 - F::new(2.0) / F::new(9.0) * t16731 + F::new(2.0) * t16735 + F::new(4.0) / F::new(3.0) * t16740 + F::new(2.0) / F::new(3.0) * t16744 + t16748 / F::new(3.0);
    (t16904, t16907, t16908, t16926)
}
