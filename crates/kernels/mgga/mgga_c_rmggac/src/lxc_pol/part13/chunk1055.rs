//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1055/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1055<F: Float>(t39528: F, t35124: F, t35128: F, t35130: F, t35132: F, t39482: F, t39486: F, t39491: F, t39493: F, t39495: F, t39497: F, t39499: F, t39514: F, t39518: F, t39523: F, t39525: F, t39531: F, t39535: F) -> F {
    let t43001 = F::cast_from(0.3193131120497015617e0_f64) * t39528;
    let t43004 = -F::cast_from(0.5454932330849068346e-1_f64) * t39482 - F::cast_from(0.2727466165424534173e-1_f64) * t39486 - F::cast_from(0.15323255961587222184e-3_f64) * t39491 - F::cast_from(0.5107751987195740728e-4_f64) * t39493 + F::cast_from(0.5107751987195740728e-4_f64) * t39495 + F::cast_from(0.1702583995731913576e-4_f64) * t39497 + F::cast_from(0.212822999466489197e-4_f64) * t39499 - F::cast_from(0.30487649791575028312e-3_f64) * t35124 + F::cast_from(0.43368970657079495308e-4_f64) * t35128 - F::cast_from(0.18183107769496894487e-1_f64) * t35130 + F::cast_from(0.3193131120497015617e0_f64) * t35132 - F::cast_from(0.1702583995731913576e-4_f64) * t39514 - F::cast_from(0.1702583995731913576e-4_f64) * t39518 + F::cast_from(0.1064114997332445985e-4_f64) * t39523 - F::cast_from(0.638468998399467591e-4_f64) * t39525 - t43001 + F::cast_from(0.35922725105591425692e0_f64) * t39531 + F::cast_from(0.47896966807455234256e0_f64) * t39535;
    t43004
}
