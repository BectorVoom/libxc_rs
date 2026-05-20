//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1301/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1301<F: Float>(t13312: F, t48: F, t10368: F, t1469: F, t2251: F, t2282: F, t4186: F, t606: F, t2258: F, t4210: F, t60: F, t10379: F, t13299: F, t13303: F, t13306: F, t1474: F, t1480: F, t2270: F, t2283: F, t2286: F, t4202: F, t4205: F, t44: F, t56: F, t614: F) -> F {
    let t13313 = t48 * t13312;
    let t13321 = t10368 * t1469 * t2251;
    let t13324 = t2282 * t4186;
    let t13325 = t13324 * t606;
    let t13328 = t4210 * t2258;
    let t13331 = t60 * t13312;
    let t13334 = F::new(220.0) / F::new(27.0) * t2270 * t1474 - F::new(40.0) / F::new(27.0) * t614 * t4202 - F::new(40.0) / F::new(9.0) * t614 * t4205 - F::new(5.0) / F::new(108.0) * t44 * t13299 + F::new(5.0) / F::new(9.0) * t44 * t13303 + F::new(5.0) / F::new(18.0) * t44 * t13306 + F::new(5.0) / F::new(6.0) * t44 * t13313 - F::new(20.0) / F::new(27.0) * t1480 * t2283 + F::new(20.0) / F::new(9.0) * t1480 * t2286 + F::new(5.0) / F::new(108.0) * t56 * t13321 + F::new(5.0) / F::new(9.0) * t56 * t13325 + F::new(5.0) / F::new(18.0) * t56 * t13328 - F::new(5.0) / F::new(6.0) * t56 * t13331 + t10379;
    t13334
}
