//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1086/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1086<F: Float>(t5840: F, t699: F, t35204: F, t35208: F, t35226: F, t35230: F, t35242: F, t35246: F, t37375: F, t45696: F, t45701: F, t45709: F, t45716: F, t45722: F, t45724: F, t45728: F, t45732: F, t45734: F, t739: F) -> (F, F) {
    let t48591 = t699 * t5840;
    let t48609 = -F::new(0.59871208509319042821e-1) * t739 * t48591 - F::new(0.425645998932978394e-4) * t45696 + F::new(0.638468998399467591e-4) * t45701 - F::new(0.38422568777328955681e-2) * t35204 + F::new(0.92232789896410962673e-3) * t35208 + F::new(0.60975299583150056624e-3) * t35226 - F::new(0.86737941314158990616e-4) * t35230 + t37375 + F::new(0.72042316457491791901e-3) * t45709 - F::new(0.10248087766267884741e-3) * t45716 + F::new(0.60975299583150056624e-3) * t35242 - F::new(0.86737941314158990616e-4) * t35246 + F::new(0.81823984962736025192e-1) * t45722 - F::new(0.16364796992547205038e0) * t45724 - F::new(0.16364796992547205038e0) * t45728 - F::new(0.16364796992547205038e0) * t45732 + F::new(0.5107751987195740728e-4) * t45734;
    (t48591, t48609)
}
