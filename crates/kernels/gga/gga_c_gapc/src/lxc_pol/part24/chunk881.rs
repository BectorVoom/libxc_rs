//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 881/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk881<F: Float>(t9009: F, t9011: F, t9014: F, t9017: F, t9021: F, t9024: F, t9027: F, t9032: F, t9034: F, t9036: F, t9038: F, t9042: F, t9044: F) -> F {
    let t10679 = -F::new(0.2471588561924985691e-3) * t9009 - F::new(0.36652500116630512966e-6) * t9011 - F::new(0.55603792169291016668e-2) * t9014 + F::new(0.15176747947735985782e-5) * t9017 - F::new(0.2698425785107458272e-5) * t9021 - F::new(0.15176747947735985782e-6) * t9024 + F::new(0.2698425785107458272e-6) * t9027 + F::new(0.14648281543675415196e-4) * t9032 - F::new(0.4637672555408563478e-4) * t9034 + F::new(0.11272120794395814009e-6) * t9036 - F::new(0.20041830772435757309e-6) * t9038 + F::new(0.11255061864162936194e-7) * t9042 + F::new(0.11255061864162936194e-6) * t9044;
    t10679
}
