//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2039/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2039<F: Float>(t101761: F, t103720: F, t103868: F, t118: F, t1310: F, t13426: F, t13514: F, t18163: F, t1843: F, t2014: F, t2089: F, t2322: F, t2372: F, t25177: F, t26210: F, t26396: F, t28586: F, t28653: F, t28683: F, t28711: F, t28737: F, t28750: F, t28926: F, t4151: F, t4254: F, t508: F, t5517: F, t651: F, t670: F, t7315: F, t7357: F, t7378: F, t7488: F, t7732: F, t7900: F, t7988: F, t8075: F, t8108: F, t95464: F, t98564: F) -> F {
    let t103873 = t8075 * t4151 - F::new(2.0) * t651 * t508 * t101761 - t26210 * t1843 - F::new(2.0) * t7357 * t5517 - F::new(2.0) * t2014 * t28926 * t7315 - F::new(2.0) * t18163 * t7988 - F::new(4.0) * t4254 * t28750 - F::new(2.0) * t28653 * t2372 - F::new(4.0) * t13426 * t7378 - F::new(2.0) * t651 * t2089 * t13514 - F::new(4.0) * t2322 * t28737 - F::new(4.0) * t651 * t28586 * t670 - F::new(4.0) * t7732 * t26396 + F::new(3.0) * t2014 * t95464 * t7900 + F::new(2.0) * t2014 * t8108 * t25177 + F::new(3.0) * t2014 * t7488 * t98564 - F::new(4.0) * t651 * t1310 * t28683 - t118 * (t103720 + t103868) - F::new(4.0) * t2322 * t28711;
    t103873
}
