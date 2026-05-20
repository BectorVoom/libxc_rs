//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2779/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2779<F: Float>(t10744: F, t14861: F, t808: F, t40791: F, t4442: F, t14468: F, t236: F, t807: F, t854: F, t10489: F, t14586: F, t14745: F, t14791: F, t1548: F, t2430: F, t2730: F, t2745: F, t2749: F, t40655: F, t40822: F, t40824: F, t40836: F, t40838: F, t40840: F, t4362: F, t4457: F, t51026: F, t51028: F, t51042: F, t51047: F, t51049: F, t51055: F, t775: F, t800: F) -> F {
    let t51058 = t10744 * t808 * t14861;
    let t51059 = F::cast_from(0.76230004213927992336e-5_f64) * t51058;
    let t51060 = t40791 * t4442;
    let t51061 = F::new(35.0) / F::new(24.0) * t51060;
    let t51070 = t807 * t236 * t854 * t14468;
    let t51072 = -F::new(7.0) / F::new(8.0) * t51026 - F::new(7.0) / F::new(16.0) * t51028 + F::new(3.0) / F::new(16.0) * t2730 * t800 * t14745 * t775 + F::new(3.0) / F::new(16.0) * t2730 * t800 * t4457 * t2430 + t2730 * t800 * t1548 * t10489 / F::new(16.0) - F::cast_from(0.45738002528356795401e-4_f64) * t51042 + F::cast_from(0.24009450146119052705e-1_f64) * t40822 - F::cast_from(0.60023625365297631762e-2_f64) * t40824 - F::cast_from(0.6098400337114239387e-4_f64) * t40836 - F::cast_from(0.12004725073059526352e-1_f64) * t51047 + F::cast_from(0.51448821741683684367e-2_f64) * t2745 * t14791 * t51049 * t2749 - F::cast_from(0.54214778996945588151e-4_f64) * t51055 + t51059 + t51061 - F::cast_from(0.51448821741683684367e-2_f64) * t4362 * t14791 * t14586 * t40655 - F::new(35.0) / F::new(72.0) * t40838 + F::new(7.0) / F::new(144.0) * t40840 + F::cast_from(0.85748036236139473944e-4_f64) * t51070;
    t51072
}
