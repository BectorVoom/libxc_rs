//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 938/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk938<F: Float>(t14555: F, t898: F, t900: F, t10835: F, t10838: F, t10839: F, t10841: F, t10843: F, t10921: F, t10923: F, t10925: F, t10927: F, t12143: F, t14484: F, t14488: F, t14491: F, t14497: F, t14501: F, t14503: F, t14507: F, t14516: F, t14520: F, t14524: F, t2265: F, t631: F) -> F {
    let t14557 = t898 * t900 * t14555;
    let t14560 = t2265 * t14484 / F::new(18.0) + F::new(2.0) / F::new(27.0) * t2265 * t14488 - F::new(2.0) / F::new(9.0) * t12143 * t14491 + t10838 + F::new(10.0) / F::new(27.0) * t10921 - t10923 / F::new(9.0) - t10925 / F::new(27.0) - t2265 * t14497 / F::new(3.0) + t2265 * t14501 + t2265 * t14503 + F::new(2.0) / F::new(3.0) * t2265 * t14507 + F::new(4.0) / F::new(9.0) * t10841 + F::new(2.0) / F::new(9.0) * t10843 - t10835 / F::new(3.0) + F::new(10.0) / F::new(9.0) * t10839 - F::new(2.0) / F::new(9.0) * t2265 * t14516 + F::new(4.0) / F::new(3.0) * t2265 * t14520 + F::new(2.0) * t2265 * t14524 + t10927 + t631 * t14557 / F::new(2.0);
    t14560
}
