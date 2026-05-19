//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 930/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk930<F: Float>(t9009: F, t9011: F, t9014: F, t9017: F, t9021: F, t9024: F, t9027: F, t9032: F, t9034: F, t9036: F, t9038: F, t9042: F, t9044: F) -> F {
    let t10679 = -F::cast_from(0.2471588561924985691e-3_f64) * t9009 - F::cast_from(0.36652500116630512966e-6_f64) * t9011 - F::cast_from(0.55603792169291016668e-2_f64) * t9014 + F::cast_from(0.15176747947735985782e-5_f64) * t9017 - F::cast_from(0.2698425785107458272e-5_f64) * t9021 - F::cast_from(0.15176747947735985782e-6_f64) * t9024 + F::cast_from(0.2698425785107458272e-6_f64) * t9027 + F::cast_from(0.14648281543675415196e-4_f64) * t9032 - F::cast_from(0.4637672555408563478e-4_f64) * t9034 + F::cast_from(0.11272120794395814009e-6_f64) * t9036 - F::cast_from(0.20041830772435757309e-6_f64) * t9038 + F::cast_from(0.11255061864162936194e-7_f64) * t9042 + F::cast_from(0.11255061864162936194e-6_f64) * t9044;
    t10679
}
