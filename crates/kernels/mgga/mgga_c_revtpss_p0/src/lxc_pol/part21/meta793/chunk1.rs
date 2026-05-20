//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2868/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2868<F: Float>(t41308: F, t41330: F, t41332: F, t41334: F, t41336: F, t41365: F, t41367: F, t52039: F, t52041: F, t52045: F, t52047: F, t52049: F, t52051: F, t52054: F, t52057: F, t52060: F, t52063: F, t52091: F, t52092: F, t52112: F) -> F {
    let t52114 = t52091 - t52092 - F::new(4.0) / F::new(3.0) * t52039 - F::new(2.0) / F::new(3.0) * t52041 - F::new(4.0) / F::new(3.0) * t52045 + F::new(4.0) / F::new(9.0) * t52047 + F::new(2.0) / F::new(9.0) * t52049 + F::new(10.0) / F::new(27.0) * t52051 - F::new(2.0) / F::new(3.0) * t52054 - F::new(2.0) / F::new(3.0) * t52057 - F::new(10.0) / F::new(9.0) * t52060 - F::new(6.0) * t52063 - F::new(2.0) / F::new(3.0) * t41365 + F::new(2.0) / F::new(9.0) * t41367 + F::new(2.0) / F::new(3.0) * t41308 - F::new(4.0) / F::new(9.0) * t41330 - F::new(8.0) / F::new(27.0) * t41332 + t41334 / F::new(9.0) + F::new(10.0) / F::new(81.0) * t41336 - F::new(6.0) * t52112;
    t52114
}
