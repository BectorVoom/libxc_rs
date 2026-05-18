//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1205/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1205<F: Float>(t91032: F, t91048: F, t91065: F, t91080: F, t193: F, t2843: F, t2862: F, t295: F, t312: F, t44436: F, t446: F, t5225: F, t5299: F, t5309: F, t5424: F, t56957: F, t840: F, t84795: F, t84797: F, t84823: F, t84825: F, t84856: F, t84880: F, t89: F) -> (F, F) {
    let t91082 = t91032 + t91048 + t91065 + t91080;
    let t91104 = t89 * t193 * t295 * t91082 * t312 / F::new(3.0) + t44436 + F::new(8.0) / F::new(27.0) * t84795 + F::new(8.0) / F::new(9.0) * t84797 + F::new(8.0) / F::new(3.0) * t84823 + F::new(4.0) / F::new(9.0) * t84825 + F::new(112.0) / F::new(81.0) * t56957 - F::new(2.0) * t446 * t840 * t5424 * t5299 - F::new(8.0) / F::new(3.0) * t84856 - F::new(16.0) / F::new(27.0) * t84880 + F::new(8.0) * t446 * t2862 * t2843 * t5225 * t5309;
    (t91082, t91104)
}
