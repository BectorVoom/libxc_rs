//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1105/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1105<F: Float>(t152815: F, t24981: F, t6317: F, t4226: F, t7584: F, t33811: F, t7512: F, t7641: F, t152770: F, t152774: F, t152779: F, t152783: F, t152788: F, t152792: F, t152797: F, t152801: F, t152804: F, t152807: F, t152810: F, t152813: F, t152817: F, t152821: F) -> (F, F, F, F) {
    let t152824 = t6317 * t24981 * t152815;
    let t152826 = t7584 * t4226;
    let t152829 = t33811 * t7512 * t7641 * t152826;
    let t152831 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t152770 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t152774 - t152779 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t152783 + t152788 / F::cast_from(12.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t152792 + t152797 / F::cast_from(12.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t152801 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t152804 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t152807 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t152810 - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t152813 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t152817 - t152821 / F::cast_from(36.0_f64) - t152824 / F::cast_from(9.0_f64) + t152829 / F::cast_from(2.0_f64);
    (t152824, t152826, t152829, t152831)
}
