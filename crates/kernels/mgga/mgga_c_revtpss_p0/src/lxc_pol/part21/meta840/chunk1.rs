//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3150/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3150<F: Float>(t56228: F, t43858: F, t43865: F, t43883: F, t43888: F, t43890: F, t43892: F, t43894: F, t43896: F, t56212: F, t56214: F, t56216: F, t56221: F, t56226: F, t56230: F, t56234: F, t56236: F, t56248: F, t56252: F, t56256: F) -> F {
    let t58090 = F::new(4.0) / F::new(9.0) * t56228;
    let t58105 = F::new(2.0) / F::new(9.0) * t56212 + F::new(4.0) / F::new(3.0) * t56214 - F::new(10.0) / F::new(27.0) * t56216 + F::new(10.0) / F::new(9.0) * t56221 + F::new(2.0) * t56226 + t58090 - t56230 / F::new(3.0) + t56234 / F::new(3.0) - F::new(28.0) / F::new(81.0) * t56236 - F::new(10.0) / F::new(81.0) * t43858 - F::new(8.0) / F::new(27.0) * t43865 + F::new(4.0) / F::new(9.0) * t43883 - F::new(28.0) / F::new(27.0) * t43888 + F::new(4.0) / F::new(9.0) * t43890 + F::new(8.0) / F::new(9.0) * t43892 - F::new(2.0) / F::new(3.0) * t43894 - t43896 / F::new(9.0) + F::new(10.0) / F::new(9.0) * t56248 + F::new(6.0) * t56252 - F::new(4.0) * t56256;
    t58105
}
