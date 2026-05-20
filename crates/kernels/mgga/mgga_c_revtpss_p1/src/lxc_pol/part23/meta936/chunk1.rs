//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3077/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3077<F: Float>(t24324: F, t3379: F, t43881: F, t68255: F, t68257: F, t68262: F, t68277: F, t81156: F, t81158: F, t81162: F, t81167: F, t81171: F, t81175: F, t81179: F, t81184: F, t81188: F, t81192: F, t81196: F, t81200: F, t81204: F, t81209: F, t81214: F) -> (F, F) {
    let t81352 = F::new(1.0) * t3379 * t24324;
    let t81379 = F::new(4.0) / F::new(9.0) * t68255 - F::new(8.0) / F::new(27.0) * t68257 + F::new(2.0) / F::new(9.0) * t81156 - F::new(2.0) / F::new(3.0) * t81158 + F::new(10.0) / F::new(9.0) * t81162 + F::new(40.0) / F::new(9.0) * t81167 + t43881 - F::new(4.0) * t81171 - F::new(8.0) * t81175 - F::new(2.0) / F::new(3.0) * t81179 - F::new(2.0) / F::new(9.0) * t81184 - F::new(2.0) / F::new(3.0) * t81188 + F::new(6.0) * t81192 + F::new(8.0) * t81196 + F::new(2.0) * t81200 + F::new(2.0) * t81204 + F::new(2.0) / F::new(3.0) * t81209 - F::new(80.0) / F::new(81.0) * t81214 - F::new(10.0) / F::new(27.0) * t68262 - F::new(2.0) / F::new(3.0) * t68277;
    (t81352, t81379)
}
